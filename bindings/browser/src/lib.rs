use log::{warn, Level};
use wasm_bindgen::prelude::wasm_bindgen;

mod server;
mod target;
mod utils;

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(Level::Warn).unwrap();
}
