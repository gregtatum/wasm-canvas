use cgmath::prelude::*;
use cgmath::{Point2, Vector2};
use dom;
use spade::rtree::RTree;
use spade::BoundingRect;
use std::cell::RefCell;
use std::rc::Rc;
use tree_node_ref::TreeNodeReference;

#[allow(unused_imports)]
use web_sys::console;

pub type MutableNodes = RefCell<Vec<Rc<RefCell<TreeNode>>>>;
type TreeNodeIndex = usize;

#[derive(Debug)]
pub struct TreeNode {
    pub start: Vector2<f64>,
    pub end: Vector2<f64>,
    pub last_drawn_end: Vector2<f64>,
    pub fully_drawn: bool,
    pub growth_length: f64,
    pub depth: i32,
    pub children: Vec<TreeNodeIndex>,
    pub grow_speed: f64,
    pub limb_length: f64,
    pub split_theta_range: f64,
    pub max_tree_depth: i32,
}

impl TreeNode {
    pub fn new(start_x: f64, start_y: f64, end_x: f64, end_y: f64, depth: i32) -> TreeNode {
        let grow_speed = 0.1 * 0.5;
        TreeNode {
            start: Vector2::new(start_x, start_y),
            end: Vector2::new(end_x, end_y),
            last_drawn_end: Vector2::new(start_x, start_y),
            growth_length: 0.0,
            fully_drawn: false,
            children: Vec::new(),
            depth: depth,
            grow_speed: random(0.02, 0.08),
            limb_length: random(0.01, 0.04),
            split_theta_range: 1.0,
            max_tree_depth: 40,
        }
    }

    pub fn draw(&mut self, nodes: &MutableNodes, page_state: &dom::PageState, force_redraw: bool) {
        let dom::PageState { width, height, ctx } = page_state;
        let l = width.min(*height);
        let w = width;
        let w2 = w * 0.5;

        let do_redraw = force_redraw || !self.fully_drawn;
        let mut end = self.end;
        let mut start = self.start;

        if !self.fully_drawn {
            if !force_redraw {
                // Do a partial redraw only if we can.
                start = self.last_drawn_end;
            }
            // Grow the line out.
            if self.growth_length == 1.0 {
                // Mark as fully drawn for the next draw call.
                self.fully_drawn = true;
            } else {
                // Compute the beginning.
                end = self
                    .start
                    .lerp(self.end, self.growth_length * cubic_out(self.growth_length))
            };
        }

        if do_redraw {
            // The lines are in terms of unit interval space, convert this into canvas device pixel
            // space, with (0, 0) centered at the top middle.
            ctx.move_to(start.x * l + w2, start.y * l);
            ctx.line_to(end.x * l + w2, end.y * l);

            // Remember the last drawn end so we can avoid re-drawing it.
            self.last_drawn_end = end;
        }

        let nodes_borrow = nodes.borrow();
        for child_node_index in &self.children {
            let mut child_node = nodes_borrow
                .get(*child_node_index)
                .expect("Child node not found for the index")
                .borrow_mut();

            // Recurse into all the child nodes
            child_node.draw(&nodes, &page_state, force_redraw)
        }
    }

    /// Go through all of the nodes, and find any that intersect, excluding the current one.
    pub fn find_intersecting_points(
        &self,
        nodes: &MutableNodes,
        r_tree: &RTree<TreeNodeReference>,
    ) -> Vec<Vector2<f64>> {
        let r_tree_intersections = r_tree.lookup_in_rectangle(&BoundingRect::from_corners(
            &Point2::new(self.start.x, self.start.y),
            &Point2::new(self.end.x, self.end.y),
        ));

        let mut intersections = Vec::new();

        // Go through all the lines and check for intersections.
        let nodes_borrow = nodes
            .try_borrow()
            .expect("Attempting to borrow the nodes to check for intersections");

        for TreeNodeReference {
            from,
            to,
            node_index,
        } in r_tree_intersections
        {
            let node_cell = nodes_borrow
                .get(*node_index)
                .expect("Got a node from a TreeNodeReference");

            match node_cell.try_borrow() {
                Ok(node) => match self.intersects(&node) {
                    Some(vec) => intersections.push(vec),
                    None => {
                        // This is the current node, as we already have mutably borrowed it.
                    }
                },
                Err(_) => {}
            }
        }
        intersections
    }

    pub fn find_nearest_intersection(
        &self,
        intersections: Vec<Vector2<f64>>,
    ) -> Option<Vector2<f64>> {
        if intersections.len() == 0 {
            return None;
        }
        intersections.iter().fold(None, |acc, x| match acc {
            Some(y) => {
                if distance_squared(&x, &self.start) < distance_squared(&y, &self.start) {
                    Some(*x)
                } else {
                    Some(y)
                }
            }
            None => Some(*x),
        })
    }

    pub fn split(&mut self, nodes: &MutableNodes, r_tree: &mut RTree<TreeNodeReference>) {
        let intersections = self.find_intersecting_points(nodes, r_tree);
        let nearest_intersection = self.find_nearest_intersection(intersections);
        let new_start = &self.end;
        let depth = self.depth + random(0.45, 1.0).round() as i32;
        let new_end = {
            // Rotate the node a bit randomly.
            let diff = self.end - self.start;
            let drift: f64 =
                js_sys::Math::random() * self.split_theta_range - self.split_theta_range * 0.5;
            let theta = diff.y.atan2(diff.x) + drift;
            Vector2::new(
                new_start.x + theta.cos() * self.limb_length,
                new_start.y + theta.sin() * self.limb_length,
            )
        };
        let new_index = {
            let mut nodes_borrow = nodes
                .try_borrow()
                .expect("Unable to checkout nodes during a split");
            nodes_borrow.len()
        };

        let new_node = {
            // Create the new node, and modify it if it intersects with any existing nodes.
            let mut new_node = TreeNode::new(new_start.x, new_start.y, new_end.x, new_end.y, depth);
            let intersections = new_node.find_intersecting_points(nodes, r_tree);
            let nearest_intersection = new_node.find_nearest_intersection(intersections);

            r_tree.insert(TreeNodeReference::from_node(&new_node, new_index));

            match nearest_intersection {
                Some(intersection) => {
                    new_node.end = intersection;
                    new_node.depth = new_node.max_tree_depth;
                }
                None => {}
            }
            new_node
        };

        {
            // Add the new node to the existing data structures.
            let mut mutable_nodes = nodes
                .try_borrow_mut()
                .expect("Unable to checkout mutables nodes for a split");
            mutable_nodes.push(Rc::new(RefCell::new(new_node)));
            self.children.push(new_index)
        }
    }

    pub fn grow(&mut self, nodes: &MutableNodes, r_tree: &mut RTree<TreeNodeReference>) {
        if self.children.len() == 0 {
            // Grow the line.
            self.growth_length = (self.growth_length + self.grow_speed).min(1.0);

            if self.growth_length == 1.0 && self.depth < self.max_tree_depth {
                self.growth_length = 1.0;
                self.split(&nodes, r_tree);
                self.split(&nodes, r_tree);
            }
        }

        for child_node_index in &mut self.children {
            let mut child_node = {
                let nodes_borrow = nodes.borrow();
                nodes_borrow
                    .get(*child_node_index)
                    .expect("Child node not found for the index")
                    .clone()
            };

            let mut child_node_borrow = child_node
                .try_borrow_mut()
                .expect("Failed to get a child node during the grow method.");

            child_node_borrow.grow(&nodes, r_tree);
        }
    }

    pub fn intersects(&self, other: &TreeNode) -> Option<Vector2<f64>> {
        check_intersection(
            self.start.x,
            self.start.y,
            self.end.x,
            self.end.y,
            other.start.x,
            other.start.y,
            other.end.x,
            other.end.y,
        )
    }
}

// Adapted from https://github.com/psalaets/line-intersect/
// Paul Salaets <psalaets@gmail.com>
// MIT License
fn check_intersection(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
    x4: f64,
    y4: f64,
) -> Option<Vector2<f64>> {
    if (x1 == x3 && y1 == y3)
        || (x1 == x4 && y1 == y4)
        || (x2 == x3 && y2 == y3)
        || (x2 == x4 && y2 == y4)
    {
        return None;
    }

    let denom = ((y4 - y3) * (x2 - x1)) - ((x4 - x3) * (y2 - y1));
    let nume_a = ((x4 - x3) * (y1 - y3)) - ((y4 - y3) * (x1 - x3));
    let nume_b = ((x2 - x1) * (y1 - y3)) - ((y2 - y1) * (x1 - x3));

    if denom == 0.0 || (nume_a == 0.0 && nume_b == 0.0) {
        return None;
    }

    let u_a = nume_a / denom;
    let u_b = nume_b / denom;

    if u_a >= 0.0 && u_a <= 1.0 && u_b >= 0.0 && u_b <= 1.0 {
        let x = (u_a * (x2 - x1)) + x1;
        let y = (u_a * (y2 - y1)) + y1;
        return Some(Vector2::new(x, y));
    }

    None
}

fn cubic_out(t: f64) -> f64 {
    let f = t - 1.0;
    return f * f * f + 1.0;
}

fn random(start: f64, end: f64) -> f64 {
    js_sys::Math::random() * (end - start) + start
}

fn distance_squared(a: &Vector2<f64>, b: &Vector2<f64>) -> f64 {
    let x = b.x - a.x;
    let y = b.y - b.x;
    x * x + y * y
}
