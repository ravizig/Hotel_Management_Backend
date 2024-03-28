use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Message<T: Serialize> {
    pub message: String,
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

pub fn response_fn<T: Serialize>(
    success: bool,
    message: String,
    data: Option<T>,
    error: String,
) -> Json<Message<T>> {
    let data = if data.is_none() { None } else { Some(data.unwrap()) };
    let error = if error.is_empty() { None } else { Some(error) };

    let message = Message {
        success,
        message,
        data,
        error,
    };

    Json(message)
}
 