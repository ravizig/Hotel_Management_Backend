use crate::{
    constants::constants,
    helpers::response_function::{response_fn, Message},
    models::user_model::User,
    repository::user_repo::UserRepo,
};
use mongodb::results::InsertOneResult;
use rocket::{serde::json::Json, State};

#[get("/")]
pub fn hello() -> String {
    "Hello World".to_string()
}

#[post("/signup", format = "application/json", data = "<new_user>")]
pub fn user_signup(
    db: &State<UserRepo>,
    new_user: Json<User>,
) -> Result<Json<Message<InsertOneResult>>, Json<Message<User>>> {
    let is_admin = if new_user.is_admin == true {
        new_user.is_admin
    } else {
        false
    };

    let data = User {
        id: None,
        username: new_user.username.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
        total_booked_rooms: vec![].into(),
        is_admin,
    };

    // Check if user already exists
    if let Ok(existing_user) = db.get_user_using_email(&new_user.email) {
        return Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::ALREADY_EXISTS_EMAIL.to_string(),
            Some(existing_user),
            constants::EMPTY.to_string(),
        ));
    }

    
    let user_detail = db.user_signup(data);

    match user_detail {
        Ok(insert_result) => Ok(response_fn(
            constants::SUCCESS_TRUE,
            constants::SIGNUP.to_string(),
            Some(insert_result),
            constants::EMPTY.to_string(),
        )),
        Err(e) => Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::SERVER_ERROR_USER.to_string(),
            None,
            e.to_string(),
        )),
    }
}

#[get("/all")]
pub fn get_all_users(
    db: &State<UserRepo>,
) -> Result<Json<Message<Vec<User>>>, Json<Message<Vec<User>>>> {
    let users = db.get_all_users();
    match users {
        Ok(users) => Ok(response_fn(
            constants::SUCCESS_TRUE,
            constants::FETCHED_USERS.to_string(),
            Some(users),
            "".to_string(),
        )),
        Err(e) => Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::SERVER_ERROR_USER.to_string(),
            None,
            e.to_string(),
        )),
    }
}
#[get("/id/<id>")]
pub fn get_user(
    db: &State<UserRepo>,
    id: String,
) -> Result<Json<Message<User>>, Json<Message<User>>> {
    if id.is_empty() {
        return Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::ID_REQUIRED.to_string(),
            None,
            constants::EMPTY.to_string(),
        ));
    }
    let user_detail = db.get_user(&id);
    match user_detail {
        Ok(user) => Ok(response_fn(
            constants::SUCCESS_TRUE,
            constants::SINGLE_USER.to_string(),
            Some(user),
            constants::EMPTY.to_string(),
        )),
        Err(e) => Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::SERVER_ERROR_USER.to_string(),
            None,
            e.to_string(),
        )),
    }
}

#[get("/email/<email>")]
pub fn get_user_using_email(
    db: &State<UserRepo>,
    email: String,
) -> Result<Json<Message<User>>, Json<Message<User>>> {
    if email.is_empty() {
        return Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::EMAIL_REQUIRED.to_string(),
            None,
            constants::EMPTY.to_string(),
        ));
    }

    match db.get_user_using_email(&email) {
        Ok(user) => Ok(response_fn(
            constants::SUCCESS_TRUE,
            constants::SINGLE_USER.to_string(),
            Some(user),
            constants::EMPTY.to_string(),
        )),
        Err(e) => Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::SERVER_ERROR_USER.to_string(),
            None,
            e.to_string(),
        )),
    }
}

#[post("/login", format = "application/json", data = "<login_data>")]
pub fn user_login(
    db: &State<UserRepo>,
    login_data: Json<User>,
) -> Result<Json<Message<String>>, Json<Message<User>>> {
    let email = login_data.email.to_string();
    let provided_password = login_data.password.to_string();

    if email.is_empty() || provided_password.is_empty() {
        return Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::REQUIRED.to_string(),
            None,
            constants::EMPTY.to_string(),
        ));
    }

    let login_details = db.user_login(&email, &provided_password);

    match login_details {
        Ok(token) => Ok(response_fn(
            constants::SUCCESS_TRUE,
            constants::LOGIN.to_string(),
            Some(token),
            constants::EMPTY.to_string(),
        )),
        Err(e) => Err(response_fn(
            constants::SUCCESS_FALSE,
            constants::SERVER_ERROR_USER.to_string(),
            None,
            e.to_string(),
        )),
    }
}
