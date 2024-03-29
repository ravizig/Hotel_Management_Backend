extern crate dotenv;

use std::{collections::HashMap, env};

use bcrypt::{hash, verify, DEFAULT_COST};

use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::InsertOneResult,
};

use rocket::serde::json::Json;
use serde::de::Error as _;

use crate::{constants::constants, models::user_model::User};

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
            .expect(constants::ERROR_CREATING_USER);

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
            .expect(constants::ERROR_FETCHING_USER);
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
            .expect(constants::ERROR_FETCHING_USER);
        if user_detail.is_none() {
            return Err(Error::custom(constants::USER_NOT_FOUND));
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
            .expect(constants::ERROR_FETCHING_USER);

        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }

    pub fn user_login(
        &self,
        email: &String,
        provided_password: &String,
    ) -> Result<String, Json<String>> {
        let user_result = self.get_user_using_email(email);

        let user = match user_result {
            Ok(user) => user,
            _ => return Err(Json(constants::USER_NOT_FOUND.to_string())),
        };

        let stored_password = user.password.clone();

        match verify(provided_password, &stored_password) {
            Ok(valid) => {
                if valid {
                    let secret_key = env::var("API_SECRET_KEY").expect(constants::FAILED_ENV);

                    let token = encode(
                        &Header::default(),
                        &user,
                        &EncodingKey::from_secret(secret_key.as_ref()),
                    );

                    match token {
                        Ok(token) => return Ok(token),
                        Err(_) => return Err(Json(constants::ERROR_TOKEN_GENERATING.to_string())),
                    }
                } else {
                    return Err(Json(constants::INVALID_PASSWORD.to_string()));
                }
            }
            Err(_) => return Err(Json(constants::ERROR_PASSWORD_VERIFY.to_string())),
        };
    }
}
