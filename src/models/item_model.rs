use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub fn default_price() -> u32 {
    0
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub description: String,
    #[serde(default = "default_price")]
    pub price: u32,
}