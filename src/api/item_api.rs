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

#[put("/update/<id>", data = "<item_detail>")]
pub fn update_item(
    db: &State<ItemRepo>,
    id: String,
    item_detail: Json<Item>,
) -> Result<Json<Message<Item>>, Json<Message<Item>>> {
    if id.is_empty() {
        return Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::ID_REQUIRED.to_string(),
            None,
            constants::EMPTY.to_string(),
        ));
    } else if item_detail.name.is_empty() {
        return Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::ITEM_NAME_REQUIRED.to_string(),
            None,
            constants::EMPTY.to_string(),
        ));
    } else if item_detail.price.le(&0) {
        return Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::ITEM_PRICE_GREATER_THAN_ZERO.to_string(),
            None,
            constants::EMPTY.to_string(),
        ));
    } else if item_detail.description.is_empty() {
        return Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::ITEM_DESCRIPTION_REQUIRED.to_string(),
            None,
            constants::EMPTY.to_string(),
        ));
    }

    let item_existing = db.get_item(id.clone());

    match item_existing {
        Ok(_) => {
            let item_detail = db.update_item(id.clone(), item_detail.clone());
            match item_detail {
                Ok(_) => {
                    let updated_item = db.get_item(id.clone());

                    if let Ok(updated_item) = updated_item {
                        return Ok(response_fn(
                            constants::SUCCESS_TRUE,
                            constants::ITEM_UPDATED.to_string(),
                            Some(updated_item),
                            constants::EMPTY.to_string(),
                        ));
                    } else {
                        return Err(response_fn(
                            constants::SUCCESS_FALSE,
                            constants::SERVER_ERROR_ITEM.to_string(),
                            None,
                            constants::EMPTY.to_string(),
                        ));
                    }
                }
                Err(e) => Err(response_fn(
                    constants::SUCCESS_FALSE,
                    constants::SERVER_ERROR_ITEM.to_string(),
                    None,
                    e.to_string(),
                )),
            }
        }
        Err(e) => Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::ITEM_NOT_FOUND.to_string(),
            None,
            e.to_string(),
        )),
    }
}

#[delete("/delete/<id>")]
pub fn delete_item(
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
    let item_existing = db.get_item(id.clone());
    match item_existing {
        Ok(item) => {
            let item_detail = db.delete_item(id.clone());
            match item_detail {
                Ok(_) => Ok(response_fn(
                    constants::SUCCESS_TRUE,
                    constants::ITEM_DELETED.to_string(),
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

        Err(e) => Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::ITEM_NOT_FOUND.to_string(),
            None,
            e.to_string(),
        )),
    }
}

#[get("/search/<search_data>")]
pub fn search_item(
    db: &State<ItemRepo>,
    search_data: String,
) -> Result<Json<Message<Vec<Item>>>, Json<Message<Vec<Item>>>> {
    let item_detail = db.search_item(search_data.clone());
    match item_detail {
        Ok(item) => {
            if item.len() > 0 {
                Ok(response_fn(
                    constants::SUCCESS_TRUE,
                    constants::SEARCH_ITEMS_FETCHED.to_string(),
                    Some(item),
                    constants::EMPTY.to_string(),
                ))
            } else {
                Ok(response_fn(
                    constants::SUCCESS_FALSE,
                    constants::SEARCH_ITEMS_NOT_FOUND.to_string(),
                    None,
                    constants::EMPTY.to_string(),
                ))
                
            }
    },
        Err(e) => Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::SERVER_ERROR_ITEM.to_string(),
            None,
            e.to_string(),
        )),
    }
}
