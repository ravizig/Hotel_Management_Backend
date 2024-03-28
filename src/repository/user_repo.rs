extern crate dotenv;

use bcrypt::{hash, verify, DEFAULT_COST};

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::InsertOneResult,
};

use rocket::serde::json::Json;
use serde::de::Error as _;

use crate::models::user_model::User;

use crate::repository::mongodb_repo::MongoRepo;
pub struct UserRepo {
    pub repo: MongoRepo, // Accepting MongoRepo directly
}

impl UserRepo {
    pub fn new(mongo_repo: &MongoRepo) -> Self {
        UserRepo {
            repo: mongo_repo.clone(),
        }
    }

    pub fn user_signup(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let is_admin = if new_user.is_admin == true {
            new_user.is_admin
        } else {
            false
        };

        let hashed_password =
            hash(new_user.password, DEFAULT_COST).expect("Error in hashing password");

        let new_doc = User {
            id: None,
            username: new_user.username,
            email: new_user.email,
            password: hashed_password,
            total_booked_rooms: vec![].into(),
            is_admin,
        };

        let user = self
            .repo
            .users_col
            .clone_with_type::<User>()
            .insert_one(new_doc, None)
            .ok()
            .expect("Error in creating user");

        return Ok(user);
    }

    pub fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .repo
            .users_col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        if user_detail.is_none() {
            return Err(Error::custom("User not found"));
        } else {
            return Ok(user_detail.unwrap());
        }
    }

    pub fn get_user_using_email(&self, email: &String) -> Result<User, Error> {
        let filter = doc! {"email": email};
        let user_detail = self
            .repo
            .users_col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        if user_detail.is_none() {
            return Err(Error::custom("User not found"));
        } else {
            return Ok(user_detail.unwrap());
        }
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursors = self
            .repo
            .users_col
            .find(None, None)
            .ok()
            .expect("Error getting list of users");

        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }

    pub fn user_login(
        &self,
        email: &String,
        provided_password: &String,
    ) -> Result<User, Json<String>> {
        let user_result = self.get_user_using_email(email);

        let user = match user_result {
            Ok(user) => user,
            _ => return Err(Json("User does not exist".to_string())),
        };

        let stored_password = user.password.clone();

        match verify(provided_password, &stored_password) {
            Ok(valid) => {
                if valid {
                    return Ok(user);
                } else {
                    return Err(Json("Invalid Password".to_string()));
                }
            }
            Err(_) => return Err(Json("Error in verifying password".to_string())),
        };
    }
}
