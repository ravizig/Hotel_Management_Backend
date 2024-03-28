use mongodb::results::InsertOneResult;
use rocket::{serde::json::Json, State};

use crate::{
    constants::constants,
    helpers::response_function::{response_fn, Message},
    models::item_model::Item,
    repository::item_repo::ItemRepo,
};

#[post("/create", data = "<new_item>")]
pub fn create_item(
    db: &State<ItemRepo>,
    new_item: Json<Item>,
) -> Result<Json<Message<InsertOneResult>>, Json<Message<Item>>> {
    let data = Item {
        id: None,
        name: new_item.name.to_owned(),
        price: new_item.price.to_owned(),
        description: new_item.description.to_owned(),
    };

    // Check if item already exists
    if let Ok(existing_item) = db.get_item_using_name(new_item.name.clone()) {
        return Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::ALREADY_EXISTS_ITEM_NAME.to_string(),
            Some(existing_item),
            constants::EMPTY.to_string(),
        ));
    }

    let item_detail = db.create_item(data);

    match item_detail {
        Ok(insert_result) => Ok(response_fn(
            constants::SUCCESS_TRUE,
            constants::ITEM_CREATED.to_string(),
            Some(insert_result),
            constants::EMPTY.to_string(),
        )),
        Err(e) => Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::SERVER_ERROR_ITEM.to_string(),
            None,
            e.to_string(),
        )),
    }
}

#[get("/all")]
pub fn get_all_items(
    db: &State<ItemRepo>,
) -> Result<Json<Message<Vec<Item>>>, Json<Message<Vec<Item>>>> {
    let items = db.get_all_items();
    match items {
        Ok(items) => Ok(response_fn(
            constants::SUCCESS_TRUE,
            constants::FETCHED_ITEMS.to_string(),
            Some(items),
            constants::EMPTY.to_string(),
        )),
        Err(e) => Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::SERVER_ERROR_ITEM.to_string(),
            None,
            e.to_string(),
        )),
    }
}

#[get("/id/<id>")]
pub fn get_item(
    db: &State<ItemRepo>,
    id: String,
) -> Result<Json<Message<Item>>, Json<Message<Item>>> {
    if id.is_empty() {
        return Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::ID_REQUIRED.to_string(),
            None,
            constants::EMPTY.to_string(),
        ));
    }
    let item_detail = db.get_item(id.clone());
    match item_detail {
        Ok(item) => Ok(response_fn(
            constants::SUCCESS_TRUE,
            constants::SINGLE_ITEM.to_string(),
            Some(item),
            constants::EMPTY.to_string(),
        )),
        Err(e) => Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::SERVER_ERROR_ITEM.to_string(),
            None,
            e.to_string(),
        )),
    }
}

#[get("/name/<item_name>")]
pub fn get_item_using_name(
    db: &State<ItemRepo>,
    item_name: String,
) -> Result<Json<Message<Item>>, Json<Message<Item>>> {
    let item_detail = db.get_item_using_name(item_name.clone());
    match item_detail {
        Ok(item) => Ok(response_fn(
            constants::SUCCESS_TRUE,
            constants::SINGLE_ITEM.to_string(),
            Some(item),
            constants::EMPTY.to_string(),
        )),
        Err(e) => Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::SERVER_ERROR_ITEM.to_string(),
            None,
            e.to_string(),
        )),
    }
}
