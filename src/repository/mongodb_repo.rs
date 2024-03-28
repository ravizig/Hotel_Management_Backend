extern crate dotenv;
use std::env;

use dotenv::dotenv;

use mongodb::
    sync::{Client, Collection}
;

use crate::models::{item_model::Item, room_model::Room, user_model::User};

#[derive(Clone)]
pub struct MongoRepo {
    pub users_col: Collection<User>,
    pub rooms_col: Collection<Room>,
    pub items_col: Collection<Item>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();

        let uri = env::var("MONGO_URI").expect("MONGO_URI must be set in .env");
        let client = Client::with_uri_str(&uri).expect("Failed to initialize MongoDB client");
        let db = client.database("Hotel-Management-DB");

        // Initialize each collection
        let users_col = db.collection("Users");
        let rooms_col = db.collection("Rooms");
        let items_col = db.collection("Items");

        // Return MongoRepo with initialized collections
        MongoRepo {
            users_col,
            rooms_col,
            items_col
        }
    }
    
}
