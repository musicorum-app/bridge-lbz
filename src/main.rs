#[macro_use]
extern crate rocket;

mod lb_routes;
mod lb_types;
mod lb_utils;
use crate::lb_routes::*;

#[get("/info")]
fn info() -> &'static str {
    "PROFANA"
}

#[get("/")]
fn index() -> &'static str {
    "PROFANA is a bridge for Listenbrainz-compatible scrobblers and Musicorum Charts."
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, info])
        .mount("/1", routes![submit_listens, validate_token])
}