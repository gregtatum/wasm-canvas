extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate console_error_panic_hook;

use std::panic;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

fn set_canvas_size() {
    let window = web_sys::window().expect("The environment should have a window object.");
    let document = window.document().expect("The window should have a document.");
    let canvas = document.get_element_by_id("canvas").expect("A canvas element must be selected.");

    let width = window.device_pixel_ratio() * window
        .inner_width()
        .expect("got the window inner width")
        .as_f64()
        .expect("turned the width to a f64");

    let height = window.device_pixel_ratio() * window
        .inner_height()
        .expect("got the window inner height")
        .as_f64()
        .expect("turned the height to a f64");

    canvas.set_attribute("width", &ToString::to_string(&width)).unwrap();
    canvas.set_attribute("height", &ToString::to_string(&height)).unwrap();
}

#[wasm_bindgen]
pub fn draw() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let window = web_sys::window().expect("The environment should have a window object.");
    let document = window.document().expect("The window should have a document.");
    let canvas = document.get_element_by_id("canvas").expect("A canvas element must be selected.");

    set_canvas_size();
    {
        let closure = Closure::wrap(Box::new(set_canvas_size) as Box<FnMut()>);
        (window.as_ref() as &web_sys::EventTarget)
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();
}
