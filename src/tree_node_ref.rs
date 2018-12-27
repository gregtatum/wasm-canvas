use cgmath::Point2;
use spade::{BoundingRect, PointN, SpatialObject};
use tree_node::TreeNode;

#[cfg(feature = "serde_serialize")]
use serde::{Deserialize, Serialize};

pub type TreeNodeReference = TreeNodeReferenceImpl<Point2<f64>>;

/// A custom tree node reference for RTree, based on:
/// https://docs.rs/spade/1.6.0/src/spade/primitives.rs.html
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde_serialize", derive(Serialize, Deserialize))]
pub struct TreeNodeReferenceImpl<V: spade::PointN> {
    pub bounding_rect: BoundingRect<V>,
    pub node_index: usize,
}

impl<V> TreeNodeReferenceImpl<V>
where
    V: PointN,
{
    /// Creates a new edge from `from` to `to`.
    pub fn from_node(node: &TreeNode, node_index: usize) -> TreeNodeReference {
        TreeNodeReferenceImpl {
            bounding_rect: BoundingRect::from_corners(
                &Point2::new(node.start.x, node.start.y),
                &Point2::new(node.end.x, node.end.y),
            ),
            node_index,
        }
    }
}

impl<V> SpatialObject for TreeNodeReferenceImpl<V>
where
    V: spade::PointN,
{
    type Point = V;

    fn mbr(&self) -> BoundingRect<V> {
        self.bounding_rect.clone()
    }

    fn distance2(&self, point: &Self::Point) -> V::Scalar {
        self.bounding_rect.min_dist2(point)
    }

    fn contains(&self, point: &Self::Point) -> bool {
        self.bounding_rect.contains_point(point)
    }
}
