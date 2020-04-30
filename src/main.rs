#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use serde::{Serialize, Deserialize};
use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: i32,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/", data = "<task>")]
fn new(task: Json<Task>) {
    println!("Got stuff.");
}

#[post("/post")]
fn posted() {
    println!("Posted /post!");
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

