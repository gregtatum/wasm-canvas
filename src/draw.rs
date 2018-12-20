extern crate console_error_panic_hook;
extern crate js_sys;
extern crate nalgebra;
extern crate wasm_bindgen;
extern crate web_sys;

use self::nalgebra::Vector2;
use dom;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
struct Line {
    start: Vector2<f64>,
    end: Vector2<f64>,
}

impl Line {
    pub fn new(start_x: f64, start_y: f64, end_x: f64, end_y: f64) -> Line {
        Line {
            start: Vector2::new(start_x, start_y),
            end: Vector2::new(end_x, end_y),
        }
    }

    pub fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        context.move_to(self.start.x, self.start.y);
        context.line_to(self.end.x, self.end.y);
    }
}

#[derive(Debug)]
struct State {
    pub lines: Vec<Line>,
}

pub fn draw() {
    let state = State {
        lines: vec![
            Line::new(10.0, 10.0, 100.0, 200.0),
            Line::new(40.0, 10.0, 100.0, 200.0),
            Line::new(60.0, 10.0, 100.0, 200.0),
        ],
    };

    let context = dom::get_context();
    let now = js_sys::Date::now();
    let width = dom::window_device_pixel_width();
    let height = dom::window_device_pixel_height();

    context.set_fill_style(&JsValue::from_str("white"));
    context.fill_rect(0.0, 0.0, width, height);

    context.set_stroke_style(&JsValue::from_str("red"));
    for line in state.lines {
        line.draw(&context);
    }

    context.stroke();
}
