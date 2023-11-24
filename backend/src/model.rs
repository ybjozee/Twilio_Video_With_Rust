use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde::ser::{Serialize as SerializeTrait, Serializer, SerializeStruct};
use serde_json::Value;
use crate::helper;

#[derive(Serialize, Deserialize)]
pub struct Claim {
    pub jti: String,
    pub iss: String,
    pub sub: String,
    pub iat: u64,
    pub nbf: u64,
    pub exp: i64,
    pub grants: VideoGrant,
}

#[derive(Serialize, Deserialize)]
pub struct VideoGrant {
    pub identity: String,
    pub video: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct NewRoomRequest {
    pub name: String,
    pub passcode: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct JoinRoomRequest {
    pub identity: Option<String>,
    pub passcode: Option<String>,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum RoomResponse {
    Success(Room),
    Error(Value),
}


pub struct Room {
    pub id: i64,
    pub name: String,
    pub passcode: Option<String>,
    pub identity: Option<String>,
    pub expiry: Option<i64>,
}

impl Room {
    pub fn is_valid_passcode(&self, passcode: Option<String>) -> bool {
        return match &self.passcode {
            None => { true }
            Some(expected_passcode) => {
                return if let Some(provided_passcode) = passcode {
                    *expected_passcode == format!("{:?}", helper::hash(provided_passcode))
                } else {
                    false
                };
            }
        };
    }
}

impl SerializeTrait for Room {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut has_passcode = false;
        if let Some(_) = &self.passcode {
            has_passcode = true;
        }
        let mut s = serializer.serialize_struct("Room", 4)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("identity", &self.identity)?;
        s.serialize_field("expiry", &self.expiry)?;
        s.serialize_field("hasPasscode", &has_passcode)?;
        s.end()
    }
}