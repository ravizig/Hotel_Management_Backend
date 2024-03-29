

// pub struct CurrentUser(pub Option<User>);

// #[rocket::async_trait]
// impl FromRequest<'static> for CurrentUser {
//     type Error = ();

//     async fn from_request(request: &'static Request<'_>) -> Outcome<Self, Self::Error> {
//         // Access user information from your authentication context (adjust accordingly)
//         let user = /* Your code to retrieve user from request */;

//         Outcome::Success(CurrentUser(user))
//     }
// }

// pub fn check_admin(mut req: Request<'_>) -> Result<Request<'_>, Status> {
//     let current_user = req.guard::<CurrentUser>().ok();

//     match current_user {
//         Some(CurrentUser(Some(user))) if user.is_admin => Ok(req),
//         _ => Err(Status::Forbidden),
//     }
// }