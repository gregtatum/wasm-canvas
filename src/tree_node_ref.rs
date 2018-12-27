use cgmath::num_traits::{one, zero};
use cgmath::{Point2, Zero};
use spade::kernels::DelaunayKernel;
use spade::primitives::EdgeSideInfo;
use spade::{BoundingRect, PointN, SpadeFloat, SpadeNum, SpatialObject, TwoDimensional};
use tree_node::TreeNode;

#[cfg(feature = "serde_serialize")]
use serde::{Deserialize, Serialize};

pub type TreeNodeReference = TreeNodeReferenceImpl<Point2<f64>>;

/// A custom tree node reference for RTree, based on:
/// https://docs.rs/spade/1.6.0/src/spade/primitives.rs.html
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde_serialize", derive(Serialize, Deserialize))]
pub struct TreeNodeReferenceImpl<V: spade::PointN> {
    pub from: V,
    pub to: V,
    pub node_index: usize,
}

// !!!!!------------------------------------------------------------------------
// Everything below here is copy/pasted from the spade repo, and hacked and slashed
// to work with this project.
// https://docs.rs/spade/1.6.0/src/spade/primitives.rs.html

impl<V> TreeNodeReferenceImpl<V>
where
    V: PointN,
{
    /// Creates a new edge from `from` to `to`.
    pub fn from_node(node: &TreeNode, node_index: usize) -> TreeNodeReference {
        TreeNodeReferenceImpl {
            from: Point2::new(node.start.x, node.start.y),
            to: Point2::new(node.end.x, node.end.y),
            node_index,
        }
    }

    /// Projects a point onto the infinite line going through the
    /// edge's start and end point and returns `true` if the projected
    /// points lies between `from` and `to`.
    pub fn is_projection_on_edge(&self, query_point: &V) -> bool {
        let (p1, p2) = (&self.from, &self.to);
        let dir = p2.sub(p1);
        let s = query_point.sub(p1).dot(&dir);
        zero::<V::Scalar>() <= s && s <= dir.length2()
    }

    /// Returns the edge's squared length.
    pub fn length2(&self) -> V::Scalar {
        let diff = self.from.sub(&self.to);
        diff.dot(&diff)
    }
}

impl<V> TreeNodeReferenceImpl<V>
where
    V: TwoDimensional,
{
    /// Determines on which side of this edge a given point lies.
    ///
    /// # Example:
    ///
    /// ```
    /// # extern crate nalgebra;
    /// # extern crate spade;
    ///
    /// use nalgebra::Point2;
    /// use spade::kernels::TrivialKernel;
    /// use spade::primitives::TreeNodeReferenceImpl;
    ///
    /// # fn main() {
    /// let e = TreeNodeReferenceImpl::new(Point2::new(0f32, 0.), Point2::new(1., 1.));
    /// assert!(e.side_query::<TrivialKernel>(&Point2::new(1.0, 0.0)).is_on_right_side());
    /// assert!(e.side_query::<TrivialKernel>(&Point2::new(0.0, 1.0)).is_on_left_side());
    /// assert!(e.side_query::<TrivialKernel>(&Point2::new(0.5, 0.5)).is_on_line());
    /// # }
    /// ```
    pub fn side_query<K: DelaunayKernel<V::Scalar>>(&self, q: &V) -> EdgeSideInfo<V::Scalar> {
        let (a, b) = (&self.from, &self.to);
        let signed_side = (b.nth(0).clone() - a.nth(0).clone())
            * (q.nth(1).clone() - a.nth(1).clone())
            - (b.nth(1).clone() - a.nth(1).clone()) * (q.nth(0).clone() - a.nth(0).clone());
        EdgeSideInfo::from_determinant(signed_side)
    }

    /// Checks if this and another edge intersect.
    ///
    /// The edges must not be collinear. Also, `true` is returned if the edges
    /// just touch each other.
    /// # Panics
    /// Panics if both lines are collinear.
    pub fn intersects_edge_non_collinear<K>(&self, other: &TreeNodeReferenceImpl<V>) -> bool
    where
        K: DelaunayKernel<V::Scalar>,
    {
        let other_from = self.side_query::<K>(&other.from);
        let other_to = self.side_query::<K>(&other.to);
        let self_from = other.side_query::<K>(&self.from);
        let self_to = other.side_query::<K>(&self.to);

        assert!(
            ![&other_from, &other_to, &self_from, &self_to]
                .iter()
                .all(|q| q.is_on_line()),
            "intersects_edge_non_collinear: Given edge is collinear."
        );

        other_from != other_to && self_from != self_to
    }
}

impl<V> TreeNodeReferenceImpl<V>
where
    V: PointN,
    V::Scalar: SpadeFloat,
{
    /// Yields the nearest point on this edge.
    pub fn nearest_point(&self, query_point: &V) -> V {
        let (p1, p2) = (&self.from, &self.to);
        let dir = p2.sub(p1);
        let s = self.project_point(query_point);
        if V::Scalar::zero() < s && s < one() {
            p1.add(&dir.mul(s))
        } else {
            if s <= V::Scalar::zero() {
                p1.clone()
            } else {
                p2.clone()
            }
        }
    }

    /// Returns the squared distance of a given point to its
    /// projection onto the infinite line going through this edge's start
    /// and end point.
    pub fn projection_distance2(&self, query_point: &V) -> V::Scalar {
        let s = self.project_point(query_point);
        let p = self.from.add(&self.to.sub(&self.from).mul(s));
        p.distance2(query_point)
    }

    /// Projects a point on this line and returns its relative position.
    ///
    /// This method will return a value between 0. and 1. (linearly interpolated) if the projected
    /// point lies between `self.from` and `self.to`, a value close to zero (due to rounding errors)
    /// if the projected point is equal to `self.from` and a value smaller than zero if the projected
    /// point lies "before" `self.from`. Analogously, a value close to 1. or greater than 1. is
    /// returned if the projected point is equal to or lies behind `self.to`.
    pub fn project_point(&self, query_point: &V) -> V::Scalar {
        let (ref p1, ref p2) = (self.from.clone(), self.to.clone());
        let dir = p2.sub(p1);
        query_point.sub(p1).dot(&dir) / dir.length2()
    }
}

impl<V: PointN> SpatialObject for TreeNodeReferenceImpl<V>
where
    V::Scalar: SpadeFloat,
{
    type Point = V;

    fn mbr(&self) -> BoundingRect<V> {
        BoundingRect::from_corners(&self.from, &self.to)
    }

    fn distance2(&self, point: &V) -> V::Scalar {
        let nn = self.nearest_point(point);
        point.sub(&nn).length2()
    }
}

/// Adds some private methods to the `PointN` trait.
pub trait PointNExtensions: PointN {
    /// Creates a new point with all components initialized to zero.
    fn new() -> Self {
        Self::from_value(zero())
    }

    /// Adds two points.
    fn add(&self, rhs: &Self) -> Self {
        self.component_wise(rhs, |l, r| l + r)
    }

    /// Substracts two points.
    fn sub(&self, rhs: &Self) -> Self {
        self.component_wise(rhs, |l, r| l - r)
    }

    /// Divides this point with a scalar value.
    fn div(&self, scalar: Self::Scalar) -> Self {
        self.map(|x| x / scalar.clone())
    }

    /// Multiplies this point with a scalar value.
    fn mul(&self, scalar: Self::Scalar) -> Self {
        self.map(|x| x * scalar.clone())
    }

    /// Applies a binary operation component wise.
    fn component_wise<F: Fn(Self::Scalar, Self::Scalar) -> Self::Scalar>(
        &self,
        rhs: &Self,
        f: F,
    ) -> Self {
        let mut result = self.clone();
        for i in 0..Self::dimensions() {
            *result.nth_mut(i) = f(self.nth(i).clone(), rhs.nth(i).clone());
        }
        result
    }

    /// Maps an unary operation to all compoenents.
    fn map<F: Fn(Self::Scalar) -> O::Scalar, O: PointN>(&self, f: F) -> O {
        let mut result = O::new();
        for i in 0..Self::dimensions() {
            *result.nth_mut(i) = f(self.nth(i).clone());
        }
        result
    }

    /// Returns a new point containing the minimum values of this and another point (componentwise)
    fn min_point(&self, rhs: &Self) -> Self {
        self.component_wise(rhs, |l, r| min_inline(l, r))
    }

    /// Returns a new point containing the maximum values of this and another point (componentwise)
    fn max_point(&self, rhs: &Self) -> Self {
        self.component_wise(rhs, |l, r| max_inline(l, r))
    }

    /// Fold operation over all point components.
    fn fold<T, F: Fn(T, Self::Scalar) -> T>(&self, mut acc: T, f: F) -> T {
        for i in 0..Self::dimensions() {
            acc = f(acc, self.nth(i).clone());
        }
        acc
    }

    /// Checks if a property holds for all components of this and another point.
    fn all_comp_wise<F: Fn(Self::Scalar, Self::Scalar) -> bool>(&self, rhs: &Self, f: F) -> bool {
        for i in 0..Self::dimensions() {
            if !f(self.nth(i).clone(), rhs.nth(i).clone()) {
                return false;
            }
        }
        true
    }

    /// Returns the point's dot product.
    fn dot(&self, rhs: &Self) -> Self::Scalar {
        self.component_wise(rhs, |l, r| l * r)
            .fold(zero(), |acc, val| acc + val)
    }

    /// Returns the point's squared length.
    fn length2(&self) -> Self::Scalar {
        self.dot(&self)
    }
}

impl<T> PointNExtensions for T where T: PointN {}

// A call to l.min(r) does not seem to be inlined, thus we define it ourselves
// This does improve performance significantly.
#[inline]
pub fn min_inline<S: SpadeNum>(a: S, b: S) -> S {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
pub fn max_inline<S: SpadeNum>(a: S, b: S) -> S {
    if a > b {
        a
    } else {
        b
    }
}
