use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

fn default_booked() -> bool {
    false
}

#[derive( Serialize, Deserialize, Debug, Clone)]
pub struct Room {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub room_number: u32,
    pub description: String,
    pub room_type: String,
    pub capacity: u8,
    pub price: u32,
    pub booked_by: Option<ObjectId>,
    #[serde(default = "default_booked")]
    pub is_booked: bool,
}

