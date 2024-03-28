use bson::{doc, extjson::de::Error, oid::ObjectId};
use mongodb::results::InsertOneResult;
use serde::de::Error as _;

use crate::models::item_model::Item;

use super::mongodb_repo::MongoRepo;

pub struct ItemRepo {
    pub repo: MongoRepo,
}

impl ItemRepo {
    pub fn new(mongo_repo: &MongoRepo) -> Self {
        ItemRepo {
            repo: mongo_repo.clone(),
        }
    }

    pub fn create_item(&self, new_item: Item) -> Result<InsertOneResult, Error> {
        let new_doc = Item {
            id: None,
            name: new_item.name,
            price: new_item.price,
            description: new_item.description,
        };

        let item = self
            .repo
            .items_col
            .clone_with_type()
            .insert_one(new_doc, None)
            .ok()
            .expect("Error in creating item");

        return Ok(item);
    }

    pub fn get_item(&self, item_id: String) -> Result<Item, Error> {
        let item_id = ObjectId::parse_str(item_id.as_str()).unwrap();

        let item = self
            .repo
            .items_col
            .clone_with_type()
            .find_one(doc! {"_id": item_id}, None)
            .ok()
            .expect("Error in getting item");

        return Ok(item.unwrap());
    }

    pub fn get_item_using_name(&self, item_name: String) -> Result<Item, Error> {
        let item = self
            .repo
            .items_col
            .clone_with_type()
            .find_one(doc! {"name": item_name}, None)
            .ok()
            .expect("Error in getting item");

        if item.is_none() {
            return Err(Error::custom("Item not found"));
        } else {
            return Ok(item.unwrap());
        }
    }

    pub fn get_all_items(&self) -> Result<Vec<Item>, Error> {
        let cursors = self
            .repo
            .items_col
            .clone_with_type()
            .find(None, None)
            .ok()
            .expect("Error in fetching items");
        let items = cursors.map(|doc| doc.unwrap()).collect();
        Ok(items)
    }
}
