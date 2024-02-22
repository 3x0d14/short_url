use crate::db::{get_url, get_urls, set_url, PgPool};
use crate::models::{NewUrl, Url};
use diesel::result::DatabaseErrorKind as DbKind;
use diesel::result::Error as DieselError;
use rocket::http::Status;

use crate::structs::{HostHeader, InboundUrl, OutboundUrl};
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::State;
use std::hash::{DefaultHasher, Hash, Hasher};
/// a test endpoint, returns the list of all urls in the database
/// # Panics
/// when anything goes wrong while fetching the database
#[get("/test")]
pub async fn test(pool: &State<PgPool>) -> Json<Vec<Url>> {
    let conn = pool.get().unwrap();
    let urls = get_urls(&conn).unwrap();
    Json(urls)
}

/// The main endpoint for the application, takes as input a url, hashes it and stores it in the database
/// while returning a link to a shortened url in Json format

#[post("/shorten", data = "<url>")]
pub fn shorten(
    url: Json<InboundUrl<'_>>,
    pool: &State<PgPool>,
    host: HostHeader,
) -> Result<Json<OutboundUrl>, Status> {
    let conn = pool.get().unwrap();
    let mut s = DefaultHasher::new();
    let destination = String::from(url.url);
    destination.hash(&mut s);
    let origin = format!("{:x}", s.finish());
    let new_url = NewUrl {
        origin: origin.clone(),
        destination: destination,
    };
    let result = Json(OutboundUrl {
        url: format!("{}/{}", host.0, origin),
    });
    match set_url(new_url, &conn) {
        Ok(_) => {}
        Err(e) => {
            // an error here is non blocking if its a duplicate key error
            match e {
                DieselError::DatabaseError(kind, _) => match kind {
                    DbKind::UniqueViolation => {}
                    _ => return Err(Status::InternalServerError),
                },
                _ => return Err(Status::InternalServerError),
            }
        }
    };
    return Ok(result);
}

/// An endpoint that redirects to the desired destination by fetching it from the database

#[get("/<code>")]
pub fn get_destination(code: String, pool: &State<PgPool>) -> Result<Redirect, Status> {
    let conn = pool.get().unwrap();
    let url: Url = get_url(code, &conn).map_err(|_| Status::NotFound)?;
    Ok(Redirect::to(url.destination))
}
