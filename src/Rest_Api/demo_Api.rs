#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_codegen;

use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello World im Rest Api!"
}

#[get("/users")]
fn get_users() -> Json<Vec<User>> {
    let users = vec![
        User { id: 1, name: "Mahesh".to_string(), email: "mahesh@example.com".to_string() },
        User { id: 2, name: "Venkat".to_string(), email: "Venkat@example.com".to_string() },
    ];

    Json(users)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, get_users])
        .launch();
}