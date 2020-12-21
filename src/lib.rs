#![recursion_limit = "1024"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod attributes;
mod cards;
mod errors;
mod sets;
mod web;

use crate::web::game_component::GameComponent;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<GameComponent>::new().mount_to_body();
}
