#![allow(dead_code)]
#![allow(unused_variables)]
pub mod dom;
pub mod draw;

extern crate console_error_panic_hook;
extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use std::panic;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console::log_1(&"Started executing the wasm code.".into());

    dom::on_window_resize(&dom::set_canvas_to_window_size);
    dom::start_raf({
        let mut state = draw::init();
        move || {
            state.width = dom::window_device_pixel_width();
            state.height = dom::window_device_pixel_height();
            draw::tick(&mut state)
        }
    });

    Ok(())
}
