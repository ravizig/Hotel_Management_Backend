mod api;
pub mod config;
pub mod helpers;
mod models;
mod repository;
pub mod constants;

#[macro_use]
extern crate rocket;

use api::{item_api::{create_item, delete_item, get_all_items, get_item, get_item_using_name, search_item, update_item}, room_api::{book_room, cancel_booking, create_room, get_all_rooms, get_room, get_room_using_number}, user_api::{get_all_users, get_user, get_user_using_email, hello, user_login, user_signup}};
use repository::{item_repo::ItemRepo, room_repo::RoomRepo, user_repo::UserRepo};
use crate::repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let mongo_db = MongoRepo::init();
    let user_repo = UserRepo::new(&mongo_db); // Create an instance of UserRepo
    let room_repo = RoomRepo::new(&mongo_db); // Create an instance of UserRepo
    let menu_repo = ItemRepo::new(&mongo_db); // Create an instance of MenuRepo

    rocket::build()
            .manage(mongo_db) // Manage MongoRepo
            .manage(user_repo) // Manage UserRepo
            .manage(room_repo) // Manage RoomRepo
            .manage(menu_repo) // Manage MenuRepo
            .mount("/", routes![hello])
            .mount("/user", routes![user_signup, user_login, get_all_users, get_user, get_user_using_email])
            .mount("/room", routes![create_room, get_room_using_number, get_room, get_all_rooms, book_room, cancel_booking])
            .mount("/item", routes![create_item, get_all_items, get_item, get_item_using_name, update_item, delete_item, search_item])
}
