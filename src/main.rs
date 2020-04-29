#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use serde::Deserialize;
use rocket_contrib::json::Json;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Deserialize)]
struct Task {
    description: String,
    complete: bool
}

#[post("/", data = "<task>")]
fn new(task: Json<Task>) {
    println!("Task {} has status {}", task.description, task.complete);
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

