// main.rs
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rand::seq::SliceRandom;
use rocket::State;

struct AppState {
    dictionary: Vec<&'static str>,
}

#[get("/get_sentence")]
fn get_sentence(State: State<AppState>) -> String {}

fn main() {
    println!("Hello, world!");
}
