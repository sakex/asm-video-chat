extern crate wasm_bindgen;
extern crate console_error_panic_hook;


mod audio;

#[macro_use]
mod js_extend;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}