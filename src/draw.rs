extern crate cgmath;
extern crate console_error_panic_hook;
extern crate js_sys;
extern crate spade;
extern crate wasm_bindgen;
extern crate web_sys;

use self::spade::rtree::RTree;
use dom::PageState;
use std::cell::RefCell;
use std::rc::Rc;
use tree_node::{MutableNodes, TreeNode};
use tree_node_ref::TreeNodeReference;
use wasm_bindgen::prelude::*;

#[allow(unused_imports)]
use web_sys::console;

// This file contains all of the initialization code for this particular visualization.
// It creates the initial conditions, and performs the higher-level update/draw calls
// for all of the components.

/// The State contains all of the state that sticks around between draw and update calls
/// for this visualization. It is owned by the requestAnimationFrame loop, and passed by
/// reference into the draw and update calls.
#[derive(Debug)]
pub struct State {
    /// All the nodes (lines) to draw for this visualization.
    pub nodes: MutableNodes,
    /// This flag gets changed when it's necessary to completely redraw the visualization.
    /// This is potentially an expensive operation, so care has been taken to limit re-draws.
    pub force_redraw: bool,
    /// Remember the current state of the page we are on, such as width and height.
    pub page: PageState,
    /// This is a data structure to help speed up intersection tests for nodes.
    pub r_tree: RTree<TreeNodeReference>,
}

/// Initialize the state for the first time. The page and canvas have already been set up, but
/// now we want to intialize the State for this particular visualization.
pub fn init(page: PageState) -> State {
    let mut r_tree = RTree::new();

    // Create the initial nodes.
    let mut tree = TreeNode::new(0.0, 0.0, 0.0, 0.0, 0);
    let l = tree.limb_length;
    // Variable naming: left, right, bottom, top - l, r, b, t
    let lt = TreeNode::new(-0.5, -0.5, -0.5 + l, -0.5 + l, 1);
    let lb = TreeNode::new(-0.5, 0.5, -0.5 + l, 0.5 - l, 1);
    let rt = TreeNode::new(0.5, -0.5, 0.5 - l, -0.5 + l, 1);
    let rb = TreeNode::new(0.5, 0.5, 0.5 - l, 0.5 - l, 1);
    {
        tree.children.push(1);
        tree.children.push(2);
        tree.children.push(3);
        tree.children.push(4);
    };
    r_tree.insert(TreeNodeReference::from_node(&tree, 0));
    r_tree.insert(TreeNodeReference::from_node(&lt, 1));
    r_tree.insert(TreeNodeReference::from_node(&lb, 2));
    r_tree.insert(TreeNodeReference::from_node(&rt, 3));
    r_tree.insert(TreeNodeReference::from_node(&rb, 4));

    let nodes = RefCell::new(vec![
        Rc::new(RefCell::new(tree)),
        Rc::new(RefCell::new(lt)),
        Rc::new(RefCell::new(lb)),
        Rc::new(RefCell::new(rt)),
        Rc::new(RefCell::new(rb)),
    ]);

    State {
        nodes,
        page,
        force_redraw: true,
        r_tree,
    }
}

/// The tick is called for every requestAnimationFrame. It delegates out to the update and
/// draw calls for the visualization.
pub fn tick(state: &mut State) {
    let base_node = state
        .nodes
        .borrow()
        .get(0)
        .expect("There must be at least 1 node.")
        .clone();

    if state.page.is_resized {
        state.force_redraw = true;
    }

    // Update:
    // Grow recursively grows all of the nodes.
    base_node.borrow_mut().grow(&state.nodes, &mut state.r_tree);;

    // Draw:
    draw_lines(&state);

    // Reset the force_redraw.
    state.force_redraw = false;
}

/// Draw all of the lines.
fn draw_lines(state: &State) {
    let ctx = &state.page.ctx;

    if state.force_redraw {
        // Only clear if we are doing a full draw.
        ctx.set_fill_style(&JsValue::from_str("#333"));
        ctx.fill_rect(0.0, 0.0, state.page.width, state.page.height);
    }

    if js_sys::Math::random() > 0.95 {
        // There are only so many bits in the color representation, and this value is
        // destructive. Only call it sparingly.
        ctx.set_fill_style(&JsValue::from_str("#33333303"));
    } else {
        ctx.set_fill_style(&JsValue::from_str("#33333302"));
    }

    ctx.fill_rect(0.0, 0.0, state.page.width, state.page.height);

    ctx.begin_path();
    ctx.set_line_width(1.5 * state.page.device_pixel_ratio);
    ctx.set_stroke_style(&JsValue::from_str("#fff"));

    let nodes_borrow = state.nodes.borrow();
    let mut base_node = nodes_borrow
        .get(0)
        .expect("There must be at least 1 node.")
        .borrow_mut();

    base_node.draw(&state.nodes, &state.page, state.force_redraw);

    ctx.stroke();
}
