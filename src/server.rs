#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]

extern crate rocket;

use rocket::{get, launch, routes};

#[get("/min/<key>")]
fn min(key: &str) -> String {
    format!("key {} y", key)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![min])
}

