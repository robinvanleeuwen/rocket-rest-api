use crate::connection::DbConn;
use crate::people::Person;
use crate::people;

use diesel::result::Error;
use std::env;
use std::result::Result;
use rocket::http::Status;
use rocket_contrib::json::Json;


#[get("/")]
fn all(connection: DbConn) -> std::result::Result<Json<Vec<Person>>, Failure> {
    people::repository::all(&connection)
        .map(|people| Json(people))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}