use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
pub struct ApiKey<'r>(&'r str);

impl<'r> ApiKey<'r> {
    pub fn get(&self) -> &str {
        self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = &'static str;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = req.headers().get_one("x-api-key");

        match token {
            Some(token) => {
                // check validity
                Outcome::Success(ApiKey(token))
            }
            // token does not exist
            None => Outcome::Error((Status::BadRequest, "Missing token")),
        }
    }
}
