use std::collections::HashMap;
use std::env;

use jsonwebtoken::{encode, EncodingKey, Header};
use jsonwebtoken::Algorithm::HS256;

use crate::helper;
use crate::model::{Claim, VideoGrant};

pub fn token(room_name: String) -> String {
    let account_sid =
        env::var("TWILIO_ACCOUNT_SID").expect("Twilio Account SID could not be retrieved.");
    let signing_key =
        env::var("TWILIO_API_KEY_SID").expect("Twilio Auth Token could not be retrieved.");
    let secret = env::var("TWILIO_API_SECRET_KEY").expect("Twilio secret key could not be retrieved");

    let mut header = Header::default();
    header.alg = HS256;
    header.typ = Some(String::from("JWT"));
    header.cty = Some(String::from("twilio-fpa;v=1"));

    let current_time = helper::current_time();

    let exp = helper::expiry();

    let mut video_grant = HashMap::new();
    video_grant.insert(String::from("room"), room_name);
    let video_claim = Claim {
        jti: format!("{}-{}", signing_key, exp),
        iss: signing_key,
        sub: account_sid,
        iat: current_time,
        nbf: current_time,
        exp,
        grants: VideoGrant {
            identity: helper::identity(),
            video: video_grant,
        },
    };

    encode(&header, &video_claim, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}
