use rocket::http::Status;
use rocket::request::Outcome;
use rocket::request::{FromRequest, Request};
use rocket::serde::{Deserialize, Serialize};
#[derive(Serialize)]
pub struct OutboundUrl {
    pub url: String,
}
#[derive(Deserialize)]
pub struct InboundUrl<'a> {
    pub url: &'a str,
}
pub struct HostHeader(pub String);
#[rocket::async_trait]
impl<'r> FromRequest<'r> for HostHeader {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        match request.headers().get_one("Host") {
            Some(h) => Outcome::Success(HostHeader(h.to_string())),
            None => Outcome::Forward(Status::NotAcceptable),
        }
    }
}
