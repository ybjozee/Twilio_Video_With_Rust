use std::env;

use axum::{Json, Router};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::{get, post};
use dotenvy::dotenv;
use serde_json::{json, Value};

use crate::helper::hash;
use crate::model::{JoinRoomRequest, NewRoomRequest, Room, RoomResponse};

mod helper;
mod model;
mod database;
mod twilio;

#[tokio::main]
async fn main() {
    dotenv().ok();
    database::setup().await;
    let frontend_url = env::var("FRONTEND_URL").expect("FRONTEND_URL could not be retrieved.");
    let app = Router::new().route("/", get(get_all_rooms))
        .route("/room", post(create_room))
        .route("/token", post(get_room_token))
        .route("/room/:id", get(get_room))
        .layer(tower_http::cors::CorsLayer::new()
            .allow_origin(frontend_url.parse::<axum::http::HeaderValue>().unwrap())
            .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
            .allow_headers([axum::http::header::CONTENT_TYPE])
        );

    let server_port = env::var("SERVER_PORT").expect("SERVER_PORT could not be retrieved.");
    let address = format!("0.0.0.0:{}", server_port);
    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_room(Json(payload): Json<NewRoomRequest>) -> (StatusCode, Json<RoomResponse>) {
    let room_name = payload.name;
    if room_name == "" {
        return (StatusCode::BAD_REQUEST, Json(RoomResponse::Error(json!({"error": "Room name cannot be empty"}))));
    }
    let mut room_passcode = payload.passcode;
    if let Some(passcode) = room_passcode {
        room_passcode = Some(format!("{:?}", hash(passcode)));
    }
    let identity = helper::identity();
    let expiry = helper::expiry();
    return if let Ok(room) = database::create_room(room_name, room_passcode, identity, expiry).await {
        (StatusCode::CREATED, Json(RoomResponse::Success(room)))
    } else {
        (StatusCode::BAD_REQUEST, Json(RoomResponse::Error(json!({"error": "Could not create new room"}))))
    };
}

async fn get_all_rooms() -> Json<Vec<Room>> {
    return if let Ok(rooms) = database::get_active_rooms().await {
        Json(rooms)
    } else { Json(vec![]) };
}

async fn get_room(Path(id): Path<String>) -> (StatusCode, Json<RoomResponse>) {
    if let Ok(room) = database::get_room(&id).await {
        return (StatusCode::OK, Json(RoomResponse::Success(room)));
    }
    return (StatusCode::NOT_FOUND, Json(RoomResponse::Error(json!({"error": "Could not find room with provided identity"}))));
}

async fn get_room_token(Json(payload): Json<JoinRoomRequest>) -> (StatusCode, Json<Value>) {
    if let Some(identity) = payload.identity {
        if let Ok(room) = database::get_room(&identity).await {
            return if room.is_valid_passcode(payload.passcode) {
                (StatusCode::OK, Json(json!({"token": twilio::token(room.name)})))
            } else {
                (StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid room passcode provided"})))
            };
        }
        return (StatusCode::NOT_FOUND, Json(json!({"error": "Could not find room with provided identity"})));
    }
    (StatusCode::BAD_REQUEST, Json(json!({"error": "room identity is required"})))
}