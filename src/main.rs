use std::{thread, time};

extern crate chrono;

mod user_handler;
use crate::user_handler::UserSpace;
mod time_handler;
use crate::time_handler::Timer;
mod tag_handler;
use crate::tag_handler::Tag;

use rand_chacha::{ChaCha20Rng, rand_core::{SeedableRng, RngCore}};
struct IdGenerator {
    generator: ChaCha20Rng,
}
impl IdGenerator {
    pub fn new() -> Self {
        IdGenerator { generator: ChaCha20Rng::from_entropy() }
    }
    pub fn next(&mut self) -> u64 {
        self.generator.next_u64()
    }
}

fn main() {
    let mut id_gen = IdGenerator::new();
    let mut timer = Timer::new(id_gen.next());
    // sleep for 10 milis (https://doc.rust-lang.org/std/thread/fn.sleep.html)
    timer.start(id_gen.next());
    thread::sleep(time::Duration::from_millis(50));
    timer.stop();
    println!("Timer part 1: {}", timer);

    timer.start(id_gen.next());
    thread::sleep(time::Duration::from_millis(100));
    println!("Timer part 2: {}", timer);
    thread::sleep(time::Duration::from_millis(50));
    timer.stop();
    println!("Timer part 2: {}", timer);
}

/*
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
use serde::{Serialize, Deserialize};

use std::sync::Mutex;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

use std::collections::HashMap;
use std::vec;
*/

/*

struct Timer {
    Vec<TimeSegment>; // store time segments
}
*/

/*
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: i32,
    // TODO: replace with Timer
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

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}
*/

/*
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/message", routes![new, update, get])
        .register(catchers![not_found])
        .manage(Mutex::new(HashMap::<ID, String>::new()))
}
*/

/*
fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
*/

/*

// The type to represent the ID of a message.
type ID = usize;

// We're going to store all of the messages here. No need for a DB.
type MessageMap = Mutex<HashMap<ID, String>>;

#[derive(Serialize, Deserialize)]
struct Message {
    id: Option<ID>,
    contents: String
}

#[post("/<id>", format = "json", data = "<message>")]
fn new(id: ID, message: Json<Message>, map: State<MessageMap>) -> JsonValue {
    let mut hashmap = map.lock().expect("map lock.");
    if hashmap.contains_key(&id) {
        json!({
            "status": "error",
            "reason": "ID exists. Try put."
        })
    } else {
        hashmap.insert(id, message.0.contents);
        json!({ "status": "ok" })
    }
}

#[put("/<id>", format = "json", data = "<message>")]
fn update(id: ID, message: Json<Message>, map: State<MessageMap>) -> Option<JsonValue> {
    let mut hashmap = map.lock().unwrap();
    if hashmap.contains_key(&id) {
        hashmap.insert(id, message.0.contents);
        Some(json!({ "status": "ok" }))
    } else {
        None
    }
}

#[get("/<id>", format = "json")]
fn get(id: ID, map: State<MessageMap>) -> Option<Json<Message>> {
    let hashmap = map.lock().unwrap();
    hashmap.get(&id).map(|contents| {
        Json(Message {
            id: Some(id),
            contents: contents.clone()
        })
    })
}

*/
