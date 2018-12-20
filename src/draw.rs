extern crate console_error_panic_hook;
extern crate js_sys;
extern crate nalgebra;
extern crate wasm_bindgen;
extern crate web_sys;

use self::nalgebra::Vector2;
use dom;
use wasm_bindgen::prelude::*;
use web_sys::console;

static GROW_SPEED: f64 = 0.1;
static LIMB_LENGTH: f64 = 0.1;
static SPLIT_THETA_RANGE: f64 = 1.0;
static MAX_TREE_DEPTH: i32 = 7;

#[derive(Debug)]
pub struct TreeNode {
    start: Vector2<f64>,
    end: Vector2<f64>,
    growth_length: f64,
    depth: i32,
    children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new(start_x: f64, start_y: f64, end_x: f64, end_y: f64, depth: i32) -> TreeNode {
        TreeNode {
            start: Vector2::new(start_x, start_y),
            end: Vector2::new(end_x, end_y),
            growth_length: 0.0,
            children: Vec::new(),
            depth: depth,
        }
    }

    pub fn draw(&self, state: &State, context: &web_sys::CanvasRenderingContext2d) {
        let l = state.width.min(state.height);
        let w = state.width;
        let w2 = w * 0.5;

        // Grow the line out.
        let end = if self.growth_length == 1.0 {
            self.end.clone()
        } else {
            self.start.lerp(&self.end, self.growth_length)
        };

        // The lines are in terms of unit interval space, convert this into canvas device pixel
        // space, with (0, 0) centered at the top middle.
        context.move_to(self.start.x * l + w2, self.start.y * l);
        context.line_to(end.x * l + w2, end.y * l);

        for child_node in &self.children {
            // Recurse into all the child nodes
            child_node.draw(&state, &context)
        }
    }

    pub fn split(&mut self) {
        let diff = self.end - self.start;
        let drift: f64 = js_sys::Math::random() * SPLIT_THETA_RANGE - SPLIT_THETA_RANGE * 0.5;
        let theta = diff.y.atan2(diff.x) + drift;

        self.children.push(TreeNode::new(
            self.end.x,
            self.end.y,
            self.end.x + theta.cos() * LIMB_LENGTH,
            self.end.y + theta.sin() * LIMB_LENGTH,
            self.depth + 1,
        ))
    }

    pub fn grow(&mut self) {
        if self.children.len() == 0 {
            // Grow the line.
            self.growth_length = (self.growth_length + GROW_SPEED).min(1.0);

            if self.growth_length == 1.0 && self.depth < MAX_TREE_DEPTH {
                self.growth_length = 1.0;
                self.split();
                self.split();
            }
        }
        for child_node in &mut self.children {
            child_node.grow()
        }
    }
}

#[derive(Debug)]
pub struct State {
    pub tree: TreeNode,
    pub width: f64,
    pub height: f64,
}

pub fn init() -> State {
    State {
        tree: TreeNode::new(0.0, 0.0, 0.0, LIMB_LENGTH, 0),
        width: 0.0,
        height: 0.0,
    }
}

pub fn tick(state: &mut State) {
    state.tree.grow();
    draw_lines(&state);
}

fn draw_lines(state: &State) {
    let context = dom::get_context();
    let now = js_sys::Date::now();

    context.set_fill_style(&JsValue::from_str("white"));
    context.fill_rect(0.0, 0.0, state.width, state.height);

    context.begin_path();
    context.set_line_width(4.0);
    context.set_stroke_style(&JsValue::from_str("black"));
    state.tree.draw(&state, &context);

    context.stroke();
}
