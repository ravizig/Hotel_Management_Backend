use bson::{doc, extjson::de::Error, oid::ObjectId};
use mongodb::results::InsertOneResult;
use rocket::serde::json::Json;
use serde::de::Error as _;

use crate::{constants::constants, models::item_model::Item};

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
            .expect(constants::ERROR_CREATING_ITEM);

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
            .expect(constants::ERROR_FETCHING_ITEM);

        return Ok(item.unwrap());
    }

    pub fn get_item_using_name(&self, item_name: String) -> Result<Item, Error> {
        let item = self
            .repo
            .items_col
            .clone_with_type()
            .find_one(doc! {"name": item_name}, None)
            .ok()
            .expect(constants::ERROR_FETCHING_ITEM);

        if item.is_none() {
            return Err(Error::custom(constants::ITEM_NOT_FOUND));
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
            .expect(constants::ERROR_FETCHING_ITEM);
        let items = cursors.map(|doc| doc.unwrap()).collect();
        Ok(items)
    }

    pub fn update_item(&self, item_id: String, item: Json<Item>) -> Result<Item, Error> {
        let item_id = ObjectId::parse_str(item_id.as_str()).unwrap();
        let filter = doc! {"_id": item_id};

        let update = doc! {
            "$set": {
                "name": item.name.clone(),
                "price": item.price.clone(),
                "description": item.description.clone(),
            }
        };

        let item = self
            .repo
            .items_col
            .clone_with_type()
            .find_one_and_update(filter, update, None)
            .ok()
            .expect(constants::ERROR_UPDATING_ITEM);

        Ok(item.unwrap())
    }

    pub fn delete_item(&self, item_id: String) -> Result<bool, Error> {
        let item_id = ObjectId::parse_str(item_id.as_str()).unwrap();
        let filter = doc! {"_id": item_id};
        let item = self
            .repo
            .items_col
            .clone_with_type::<Item>()
            .delete_one(filter, None)
            .ok()
            .expect(constants::ERROR_DELETING_ITEM);

        Ok(item.deleted_count > 0)
    }

    pub fn search_item(&self, item_name: String) -> Result<Vec<Item>, Error> {
        
        let regex_pattern = format!(".*{}.*", regex::escape(&item_name));

        let filter = doc! {
            "$or": [
            {
                "name": {
                    "$regex": &regex_pattern,
                    "$options": "i"
                }
            },
            {
                "description": {
                    "$regex": &regex_pattern,
                    "$options": "i"
                }
            }
        ]
        };

        let items = self
            .repo
            .items_col
            .clone_with_type()
            .find(Some(filter), None)
            .ok()
            .expect(constants::ERROR_SEARCHING_ITEM);

        let items = items.map(|doc| doc.unwrap()).collect();
        Ok(items)
    }
}
