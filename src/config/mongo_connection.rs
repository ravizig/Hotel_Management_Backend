// use mongodb::
//     sync::Client
// ;
// use dotenv::dotenv;
// use std::env;


// pub fn connect_to_mongo() -> Result<Client, Box<dyn std::error::Error>> {
//     dotenv().ok();

//     let uri = env::var("MONGO_URI").expect("MONGO_URI must be set in .env");
//     let client = Client::with_uri_str(&uri)?;
//     let db = client.database("Hotel-Management-DB");

//     Ok(client)
// }