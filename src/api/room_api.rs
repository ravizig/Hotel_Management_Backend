use std::collections::HashMap;

use mongodb::results::InsertOneResult;
use rocket::{serde::json::Json, State};
use serde::Deserialize;

use crate::{
    constants::constants,
    helpers::response_function::{response_fn, Message},
    models::room_model::Room,
    repository::{room_repo::RoomRepo, user_repo::UserRepo},
};

// Define a struct to represent the data sent in the request body
#[derive(Debug, Deserialize)]
pub struct BookingData {
    pub booked_by: String,
    pub room_number: u32,
}

#[post("/create", data = "<new_room>")]
pub fn create_room(
    db: &State<RoomRepo>,
    new_room: Json<Room>,
) -> Result<Json<Message<InsertOneResult>>, Json<Message<Room>>> {
    let is_booked = if new_room.is_booked == true {
        new_room.is_booked.to_owned()
    } else {
        false
    };

    let data = Room {
        id: None,
        room_number: new_room.room_number.to_owned(),
        description: new_room.description.to_owned(),
        room_type: new_room.room_type.to_owned(),
        capacity: new_room.capacity.to_owned(),
        price: new_room.price.to_owned(),
        booked_by: None,
        is_booked,
    };

    // Check if room already exists
    if let Ok(existing_room) = db.get_room_using_room_number(&new_room.room_number) {
        return Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::ALREADY_EXISTS_ROOM_NUMBER.to_string(),
            Some(existing_room),
            constants::EMPTY.to_string(),
        ));
    }

    let room_detail = db.create_room(data);

    match room_detail {
        Ok(insert_result) => Ok(response_fn(
            constants::SUCCESS_TRUE,
            constants::ROOM_CREATED.to_string(),
            Some(insert_result),
            constants::EMPTY.to_string(),
        )),
        Err(e) => Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::SERVER_ERROR_ROOM.to_string(),
            None,
            e.to_string(),
        )),
    }
}

#[get("/room_number/<room_number>")]
pub fn get_room_using_number(
    db: &State<RoomRepo>,
    room_number: u32,
) -> Result<Json<Message<Room>>, Json<Message<Room>>> {
    let room_detail = db.get_room_using_room_number(&room_number);
    match room_detail {
        Ok(room) => Ok(response_fn(
            constants::SUCCESS_TRUE,
            constants::SINGLE_ROOM.to_string(),
            Some(room),
            constants::EMPTY.to_string(),
        )),
        Err(e) => Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::SERVER_ERROR_ROOM.to_string(),
            None,
            e.to_string(),
        )),
    }
}

#[get("/id/<room_id>")]
pub fn get_room(
    db: &State<RoomRepo>,
    user_repo: &State<UserRepo>,
    room_id: String,
) -> Result<Json<Message<HashMap<String, serde_json::Value>>>, Json<Message<Room>>> {
    let room_detail = db.get_room(&room_id);

    match room_detail {
        Ok(room) => {
            // Check if the room is booked
            if room.is_booked {
                let user_detail = user_repo.get_user(&room.booked_by.unwrap().to_string());
                match user_detail {
                    Ok(user) => {
                        // Construct a HashMap to represent the additional field ('booked_by')
                        let mut response_data = HashMap::new();
                        response_data
                            .insert("room".to_string(), serde_json::to_value(&room).unwrap());
                        response_data.insert(
                            "booked_by".to_string(),
                            serde_json::to_value(&user).unwrap(),
                        );

                        Ok(response_fn(
                            constants::SUCCESS_TRUE,
                            constants::SINGLE_ROOM.to_string(),
                            Some(response_data), // Include the HashMap in the response
                            constants::EMPTY.to_string(),
                        ))
                    }
                    Err(e) => Err(response_fn(
                        constants::SUCCESS_FALSE,
                        constants::SERVER_ERROR_USER.to_string(),
                        None,
                        e.to_string(),
                    )),
                }
            } else {
                let mut response_data = HashMap::new();
                response_data.insert("room".to_string(), serde_json::to_value(&room).unwrap());

                Ok(response_fn(
                    constants::SUCCESS_TRUE,
                    constants::SINGLE_ROOM.to_string(),
                    Some(response_data),
                    constants::EMPTY.to_string(),
                ))
            }
        }
        Err(e) => Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::SERVER_ERROR_ROOM.to_string(),
            None,
            e.to_string(),
        )),
    }
}

#[get("/all")]
pub fn get_all_rooms(
    db: &State<RoomRepo>,
) -> Result<Json<Message<Vec<Room>>>, Json<Message<Vec<Room>>>> {
    let rooms = db.get_all_rooms();
    match rooms {
        Ok(rooms) => Ok(response_fn(
            constants::SUCCESS_TRUE,
            constants::FETCHED_ROOMS.to_string(),
            Some(rooms),
            constants::EMPTY.to_string(),
        )),
        Err(e) => Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::SERVER_ERROR_ROOM.to_string(),
            None,
            e.to_string(),
        )),
    }
}

#[put("/book", data = "<booking_data>")]
pub fn book_room(
    db: &State<RoomRepo>,
    user_repo: &State<UserRepo>,
    booking_data: Json<BookingData>,
) -> Result<Json<Message<HashMap<String, serde_json::Value>>>, Json<Message<Room>>> {
    let user_id = &booking_data.booked_by;
    let room_number = booking_data.room_number;

    let room_detail = db.get_room_using_room_number(&room_number);

    match room_detail {
        Ok(room) => {
            if room.is_booked == true {
                return Err(response_fn(
                    constants::SUCCESS_FALSE,
                    constants::ROOM_ALREADY_BOOKED.to_string(),
                    Some(room),
                    constants::EMPTY.to_string(),
                ));
            } else {
                let update_result = db.book_room(room.id.unwrap(), user_id.clone());

                match update_result {
                    Ok(update_result) => {
                        // Fetch user details based on user_id
                        let user_detail = user_repo.get_user(&user_id);
                        match user_detail {
                            Ok(user) => {
                                let updated_room_details =
                                    db.get_room_using_room_number(&room_number);

                                let mut response_data = HashMap::new();

                                match updated_room_details {
                                    Ok(updated_room_details) => {
                                        response_data.insert(
                                            "booked_by".to_string(),
                                            serde_json::to_value(&user).unwrap(),
                                        );
                                        response_data.insert(
                                            "updated_result".to_string(),
                                            serde_json::to_value(&update_result).unwrap(),
                                        );
                                        response_data.insert(
                                            "updated_room_details".to_string(),
                                            serde_json::to_value(&updated_room_details).unwrap(),
                                        );

                                        Ok(response_fn(
                                            constants::SUCCESS_TRUE,
                                            constants::ROOM_BOOKED.to_string(),
                                            Some(response_data),
                                            constants::EMPTY.to_string(),
                                        ))
                                    }
                                    Err(e) => Err(response_fn(
                                        constants::SUCCESS_FALSE,
                                        constants::SERVER_ERROR_ROOM.to_string(),
                                        None,
                                        e.to_string(),
                                    )),
                                }
                            }
                            Err(e) => Err(response_fn(
                                constants::SUCCESS_FALSE,
                                constants::SERVER_ERROR_USER.to_string(),
                                None,
                                e.to_string(),
                            )),
                        }
                    }

                    Err(e) => Err(response_fn(
                        constants::SUCCESS_FALSE,
                        constants::SERVER_ERROR_ROOM.to_string(),
                        None,
                        e.to_string(),
                    )),
                }
            }
        }
        Err(e) => Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::SERVER_ERROR_ROOM.to_string(),
            None,
            e.to_string(),
        )),
    }
}

#[put("/cancel_booking", data = "<booking_data>")]
pub fn cancel_booking(
    db: &State<RoomRepo>,
    user_repo: &State<UserRepo>,
    booking_data: Json<BookingData>,
) -> Result<Json<Message<HashMap<String, serde_json::Value>>>, Json<Message<Room>>> {
    let user_id = &booking_data.booked_by;
    let room_number = booking_data.room_number;

    let room_detail = db.get_room_using_room_number(&room_number);

    match room_detail {
        Ok(room) => {
            if room.is_booked == false {
                return Err(response_fn(
                    constants::SUCCESS_FALSE,
                    constants::ROOM_NOT_BOOKED.to_string(),
                    Some(room),
                    constants::EMPTY.to_string(),
                ));
            } else {
                let update_result = db.cancel_booking(room.id.unwrap(), user_id.clone());

                match update_result {
                    Ok(update_result) => {
                        // Fetch user details based on user_id
                        let user_detail = user_repo.get_user(&user_id);
                        match user_detail {
                            Ok(user) => {
                                let updated_room_details =
                                    db.get_room_using_room_number(&room_number);

                                let mut response_data = HashMap::new();

                                match updated_room_details {
                                    Ok(updated_room_details) => {
                                        response_data.insert(
                                            "canceled_by".to_string(),
                                            serde_json::to_value(&user).unwrap(),
                                        );
                                        response_data.insert(
                                            "update_result".to_string(),
                                            serde_json::to_value(&update_result).unwrap(),
                                        );
                                        response_data.insert(
                                            "updated_room_details".to_string(),
                                            serde_json::to_value(&updated_room_details).unwrap(),
                                        );

                                        Ok(response_fn(
                                            constants::SUCCESS_TRUE,
                                            constants::ROOM_CANCELED.to_string(),
                                            Some(response_data),
                                            constants::EMPTY.to_string(),
                                        ))
                                    }
                                    Err(e) => Err(response_fn(
                                        constants::SUCCESS_FALSE,
                                        constants::SERVER_ERROR_ROOM.to_string(),
                                        None,
                                        e.to_string(),
                                    )),
                                }
                            }
                            Err(e) => Err(response_fn(
                                constants::SUCCESS_FALSE,
                                constants::SERVER_ERROR_USER.to_string(),
                                None,
                                e.to_string(),
                            )),
                        }
                    }

                    Err(e) => Err(response_fn(
                        constants::SUCCESS_FALSE,
                        constants::SERVER_ERROR_ROOM.to_string(),
                        None,
                        e.to_string(),
                    )),
                }
            }
        }
        Err(e) => Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::SERVER_ERROR_ROOM.to_string(),
            None,
            e.to_string(),
        )),
    }
}
