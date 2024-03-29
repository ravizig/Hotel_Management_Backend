extern crate dotenv;

use bson::oid::ObjectId;
use mongodb::{
    bson::{doc, extjson::de::Error},
    results::{InsertOneResult, UpdateResult},
};

use serde::de::Error as _;

use crate::{constants::constants, models::{room_model::Room, user_model::User}};

use crate::repository::mongodb_repo::MongoRepo;
use crate::repository::user_repo::UserRepo;
pub struct RoomRepo {
    pub repo: MongoRepo,
}

impl RoomRepo {
    pub fn new(mongo_repo: &MongoRepo) -> Self {
        RoomRepo {
            repo: mongo_repo.clone(),
        }
    }

    pub fn create_room(&self, new_room: Room) -> Result<InsertOneResult, Error> {
        let is_booked = if new_room.is_booked == true {
            new_room.is_booked
        } else {
            false
        };

        let new_doc = Room {
            id: None,
            room_number: new_room.room_number,
            description: new_room.description,
            room_type: new_room.room_type,
            capacity: new_room.capacity,
            price: new_room.price,
            booked_by: None,
            is_booked,
        };

        let room = self
            .repo
            .rooms_col
            .clone_with_type::<Room>()
            .insert_one(new_doc, None)
            .ok()
            .expect(constants::ERROR_CREATING_ROOM);

        return Ok(room);
    }

    pub fn get_all_rooms(&self) -> Result<Vec<Room>, Error> {
        let cursors = self
            .repo
            .rooms_col
            .clone_with_type::<Room>()
            .find(None, None)
            .ok()
            .expect(constants::ERROR_FETCHING_ROOM);
        let rooms = cursors.map(|doc| doc.unwrap()).collect();
        Ok(rooms)
    }

    pub fn get_room(&self, id: &String) -> Result<Room, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let room_detail = self
            .repo
            .rooms_col
            .clone_with_type::<Room>()
            .find_one(filter, None)
            .ok()
            .expect(constants::ERROR_FETCHING_ROOM);

        if room_detail.is_none() {
            return Err(Error::custom(constants::ROOM_NOT_FOUND));
        } else {
            return Ok(room_detail.unwrap());
        }
    }

    pub fn get_room_using_room_number(&self, room_number: &u32) -> Result<Room, Error> {
        let filter = doc! {"room_number": room_number};

        let room_detail = self
            .repo
            .rooms_col
            .clone_with_type::<Room>()
            .find_one(filter, None)
            .ok()
            .expect(constants::ERROR_FETCHING_ROOM);

        if room_detail.is_none() {
            return Err(Error::custom(constants::ROOM_NOT_FOUND));
        } else {
            return Ok(room_detail.unwrap());
        }
    }

    pub fn book_room(&self, room_id: ObjectId, user_id: String) -> Result<(UpdateResult, UpdateResult), Error> {
        let user_id = ObjectId::parse_str(&user_id)?;
        let filter = doc! {"_id": room_id};
        let filter_user = doc! {"_id": user_id};

        let user_detail = UserRepo::new(&self.repo).get_user(&user_id.to_string());
        if let Ok(mut user) = user_detail {
            // Add the room_id to the booked_rooms array
            if let Some(booked_rooms) = &mut user.total_booked_rooms {
                booked_rooms.push(room_id.clone());
            } else {
                user.total_booked_rooms = Some(vec![room_id.clone()]);
            }

            // Construct the update document for the user
            let update_user_doc = doc! {
                "$set": {
                    "total_booked_rooms": user.total_booked_rooms,
                }
            };

            // Update the user document
            let update_user = self
                .repo
                .users_col
                .clone_with_type::<User>()
                .update_one(filter_user, update_user_doc, None)
                .ok()
                .expect(constants::ERROR_UPDATING_USER);

            // Construct the update document for the room
            let update_doc = doc! {
                "$set": {
                    "booked_by": Some(user_id.clone()),
                    "is_booked": true,
                }
            };

            // Update the room document
            let update_result = self
                .repo
                .rooms_col
                .clone_with_type::<Room>()
                .update_one(filter, update_doc, None)
                .ok()
                .expect(constants::ERROR_UPDATING_ROOM);

            Ok((update_result, update_user))
        } else {
            Err(Error::custom(constants::USER_NOT_FOUND))
        }
    }

    pub fn cancel_booking(
        &self,
        room_id: ObjectId,
        user_id: String,
    ) -> Result<(UpdateResult, UpdateResult), Error> {
        let user_id = ObjectId::parse_str(&user_id)?;
        let filter = doc! {"_id": room_id};

        let user_detail = UserRepo::new(&self.repo).get_user(&user_id.to_string());

        if let Ok(mut user) = user_detail {
            
            // Remove the room_id from the booked_rooms array
            if let Some(booked_rooms) = &mut user.total_booked_rooms {
                booked_rooms.retain(|id| *id != room_id);
            }

            // Construct the update document for the user
            let update_user_doc = doc! {
                "$set": {
                    "total_booked_rooms": user.total_booked_rooms,
                }
            };

            // Update the user document
            let update_user = self
                .repo
                .users_col
                .clone_with_type::<User>()
                .update_one(doc! { "_id": user_id.clone() }, update_user_doc, None)
                .ok()
                .expect(constants::ERROR_UPDATING_ROOM);

            // Construct the update document for the room
            let update_doc = doc! {
                "$set": {
                    "booked_by": null,
                    "is_booked": false,
                }
            };

            // Update the room document
            let update_result = self
                .repo
                .rooms_col
                .clone_with_type::<Room>()
                .update_one(filter, update_doc, None)
                .ok()
                .expect(constants::ERROR_UPDATING_ROOM);

            Ok((update_result, update_user))

        } else {
            Err(Error::custom(constants::USER_NOT_FOUND))
        }
    }

}
