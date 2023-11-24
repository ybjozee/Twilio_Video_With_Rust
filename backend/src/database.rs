use std::env;

use sqlx::{Error, migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};

use crate::helper::current_time;
use crate::model::Room;

pub async fn setup() {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL could not be retrieved.");
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        println!("Creating database {}", db_url);
        match Sqlite::create_database(&db_url).await {
            Ok(_) => {
                println!("Create db success");
                let db = SqlitePool::connect(&db_url).await.unwrap();
                let result = sqlx::query("CREATE TABLE IF NOT EXISTS room (id INTEGER PRIMARY KEY NOT NULL, name VARCHAR(250) NOT NULL, passcode VARCHAR(250), identity VARCHAR(250) NOT NULL, expiry INTEGER NOT NULL);")
                    .execute(&db).await.unwrap();
                println!("Create user table result: {:?}", result);
            }
            Err(error) => panic!("error: {}", error),
        }
    }
}

pub async fn create_room(name: String, passcode: Option<String>, identity: String, expiry: i64) -> Result<Room, Error> {
    let db = get_db_pool().await;
    sqlx::query_as!(Room, "INSERT INTO room (name, passcode, identity, expiry) VALUES ($1, $2, $3, $4) returning *", name, passcode, identity, expiry)
        .fetch_one(&db)
        .await
}

pub async fn get_db_pool() -> Pool<Sqlite> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL could not be retrieved.");
    SqlitePool::connect(&db_url).await.unwrap()
}

pub async fn get_active_rooms() -> Result<Vec<Room>, Error> {
    let db = get_db_pool().await;
    let current_time = current_time() as i64;
    sqlx::query_as!(Room, "SELECT * FROM room where expiry > $1", current_time).fetch_all(&db).await
}

pub async fn get_room(identity: &String) -> Result<Room, Error> {
    let db = get_db_pool().await;
    let room = sqlx::query_as!(Room, "SELECT * FROM room where identity = $1", identity).fetch_one(&db).await;
    return room;
}