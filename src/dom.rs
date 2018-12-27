use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// This module provides easy interfaces into the web_sys library.

#[derive(Debug)]
pub struct PageState {
    pub width: f64,
    pub height: f64,
    pub device_pixel_ratio: f64,
    pub is_resized: bool,
    pub ctx: web_sys::CanvasRenderingContext2d,
}

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn request_animation_frame(f: &Closure<FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

pub fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

pub fn window_device_pixel_width() -> f64 {
    window().device_pixel_ratio()
        * window()
            .inner_width()
            .expect("got the window inner width")
            .as_f64()
            .expect("turned the width to a f64")
}

pub fn window_device_pixel_height() -> f64 {
    window().device_pixel_ratio()
        * window()
            .inner_height()
            .expect("got the window inner width")
            .as_f64()
            .expect("turned the width to a f64")
}

#[derive(Deserialize, Serialize)]
struct ContextOptions {
    alpha: bool,
}

pub fn get_context() -> web_sys::CanvasRenderingContext2d {
    let canvas = document()
        .get_element_by_id("canvas")
        .expect("A canvas element must be selected.");

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas
        .get_context_with_context_options(
            "2d",
            &JsValue::from_serde(&ContextOptions { alpha: false }).unwrap(),
        )
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

pub fn set_canvas_to_window_size() {
    let canvas = document()
        .get_element_by_id("canvas")
        .expect("A canvas element must be selected.");

    canvas
        .set_attribute("width", &ToString::to_string(&window_device_pixel_width()))
        .unwrap();
    canvas
        .set_attribute(
            "height",
            &ToString::to_string(&window_device_pixel_height()),
        )
        .unwrap();
}

pub fn start_raf<F>(mut callback: F)
where
    // The function passed in is mutable, and each reference has to have a lifetime
    // at least equal to 'static. The function itself is not static.
    F: FnMut() + 'static,
{
    // Create a self-referential reference counted cell. This cell contains our
    // closure which will be looped over, and kept alive.
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        callback();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<FnMut()>));

    // Kick off the raf loop.
    request_animation_frame(g.borrow().as_ref().unwrap());
}

pub fn on_window_resize<F>(callback: F)
where
    F: FnMut() + 'static,
{
    let closure = Closure::wrap(Box::new(callback) as Box<FnMut()>);
    (window().as_ref() as &web_sys::EventTarget)
        .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
}
