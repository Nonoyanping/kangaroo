// main.rs
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rand::seq::SliceRandom;
use rocket::{response::content, State};

struct AppState {
    dictionary: Vec<&'static str>,
}

#[get("/get_sentence")]
fn get_sentence(state: State<AppState>) -> String {
    let sentence = state
        .dictionary
        .choose(&mut rand::thread_rng())
        .unwrap_or(&"Default sentence");
    sentence.to_string()
}

#[get("/custom_test/<content>")]
fn custom_test(content: String) -> String {
    content
}

fn main() {
    println!("Hello, world!");
}
