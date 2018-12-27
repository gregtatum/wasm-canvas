extern crate cgmath;
extern crate console_error_panic_hook;
extern crate js_sys;
extern crate spade;
extern crate wasm_bindgen;
extern crate web_sys;

use self::spade::rtree::RTree;
use dom;
use std::cell::RefCell;
use std::rc::Rc;
use tree_node::{MutableNodes, TreeNode};
use tree_node_ref::TreeNodeReference;
use wasm_bindgen::prelude::*;

#[allow(unused_imports)]
use web_sys::console;

#[derive(Debug)]
pub struct State {
    pub nodes: MutableNodes,
    pub force_redraw: bool,
    pub page: dom::PageState,
    pub r_tree: RTree<TreeNodeReference>,
}

pub fn init(page: dom::PageState) -> State {
    let mut r_tree = RTree::new();
    let mut tree = TreeNode::new(0.0, 0.0, 0.0, 0.0, 0);
    let l0 = tree.limb_length;
    let l1 = 1.0 - tree.limb_length;
    let a = TreeNode::new(-0.5, 0.0, -0.5 + l0, l0, 0);
    let b = TreeNode::new(-0.5, 1.0, -0.5 + l0, l1, 0);
    let c = TreeNode::new(0.5, 1.0, 0.5 - l0, l1, 0);
    let d = TreeNode::new(0.5, 0.0, 0.5 - l0, l0, 0);
    {
        tree.children.push(1);
        tree.children.push(2);
        tree.children.push(3);
        tree.children.push(4);
    };
    r_tree.insert(TreeNodeReference::from_node(&tree, 0));
    r_tree.insert(TreeNodeReference::from_node(&a, 1));
    r_tree.insert(TreeNodeReference::from_node(&b, 2));
    r_tree.insert(TreeNodeReference::from_node(&c, 3));
    r_tree.insert(TreeNodeReference::from_node(&d, 4));

    let nodes = RefCell::new(vec![
        Rc::new(RefCell::new(tree)),
        Rc::new(RefCell::new(a)),
        Rc::new(RefCell::new(b)),
        Rc::new(RefCell::new(c)),
        Rc::new(RefCell::new(d)),
    ]);
    let force_redraw = true;

    State {
        nodes,
        page,
        force_redraw,
        r_tree,
    }
}

pub fn tick(state: &mut State) {
    let base_node = state
        .nodes
        .borrow()
        .get(0)
        .expect("There must be at least 1 node.")
        .clone();

    base_node.borrow_mut().grow(&state.nodes, &mut state.r_tree);;
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
