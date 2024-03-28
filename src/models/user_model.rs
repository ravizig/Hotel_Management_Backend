
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

fn default_role() -> bool {
    false
}

fn default_username() -> String {
    "user".to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(default = "default_username")]
    pub username: String,
    pub email: String,
    pub password: String,
    pub total_booked_rooms: Option<Vec<ObjectId>>,
    #[serde(default = "default_role")]
    pub is_admin: bool,
}

