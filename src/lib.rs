pub mod dom;
pub mod draw;
pub mod tree_node;
pub mod tree_node_ref;

#[macro_use]
extern crate serde_derive;
extern crate cgmath;
extern crate console_error_panic_hook;
extern crate js_sys;
extern crate spade;
extern crate wasm_bindgen;
extern crate web_sys;

use std::cell::RefCell;
use std::panic;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::console;

/// This module handles the high-level initialization.

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console::log_1(&"Started executing the wasm code.".into());

    // Track page resizing across callback.
    let page_is_resized = Rc::new(RefCell::new(true));

    // Handle window resizes.
    dom::on_window_resize({
        let page_is_resized_refcell = page_is_resized.clone();
        // Set the canvas size and then add an event listener to resize on window resize.
        dom::set_canvas_to_window_size();
        move || {
            dom::set_canvas_to_window_size();
            *page_is_resized_refcell.borrow_mut() = true;
        }
    });

    dom::start_raf({
        let mut state = draw::init(dom::PageState {
            width: dom::window_device_pixel_width(),
            height: dom::window_device_pixel_height(),
            device_pixel_ratio: dom::window().device_pixel_ratio(),
            is_resized: true,
            ctx: dom::get_context(),
        });

        move || {
            let is_resized = page_is_resized.borrow().clone();
            if is_resized {
                state.page.width = dom::window_device_pixel_width();
                state.page.height = dom::window_device_pixel_height();
                state.page.device_pixel_ratio = dom::window().device_pixel_ratio();
                state.page.is_resized = true;
                *page_is_resized.borrow_mut() = false;
            }

            draw::tick(&mut state);
            state.page.is_resized = false;
        }
    });

    Ok(())
}
