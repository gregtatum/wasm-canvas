extern crate console_error_panic_hook;
extern crate js_sys;
extern crate nalgebra;
extern crate wasm_bindgen;
extern crate web_sys;

use dom;
use lines::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::console;

type MutableNodes = RefCell<Vec<Rc<RefCell<TreeNode>>>>;

#[derive(Debug)]
pub struct State {
    pub nodes: MutableNodes,
    pub force_redraw: bool,
    pub page: dom::PageState,
}

pub fn init(page: dom::PageState) -> State {
    let mut tree = TreeNode::new(0.0, 0.0, 0.0, 0.0, 0);
    tree.end.y = tree.limb_length;
    let nodes = RefCell::new(vec![Rc::new(RefCell::new(tree))]);
    let force_redraw = true;

    State {
        nodes,
        page,
        force_redraw,
    }
}

pub fn tick(state: &mut State) {
    let base_node = state
        .nodes
        .borrow()
        .get(0)
        .expect("There must be at least 1 node.")
        .clone();

    base_node.borrow_mut().grow(&state.nodes);;
    draw_lines(&state);
    state.force_redraw = false;
}

fn draw_lines(state: &State) {
    let context = dom::get_context();
    let now = js_sys::Date::now();

    if state.force_redraw {
        // Only clear if we are doing a full draw.
        context.set_fill_style(&JsValue::from_str("#333"));
        context.fill_rect(0.0, 0.0, state.page.width, state.page.height);
    }

    context.begin_path();
    context.set_line_width(4.0);
    context.set_stroke_style(&JsValue::from_str("#fff"));

    let nodes_borrow = state.nodes.borrow();
    let mut base_node = nodes_borrow
        .get(0)
        .expect("There must be at least 1 node.")
        .borrow_mut();

    base_node.draw(&state.nodes, &state.page, state.force_redraw);

    context.stroke();
}
