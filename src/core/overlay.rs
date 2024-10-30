//! This module contains functionality to construct and manage overlays, which are used to perform
//! boolean operations (union, intersection, etc.) on polygons. It provides structures and methods to
//! manage subject and clip polygons and convert them into graphs for further operations.

use i_float::int::point::IntPoint;
use i_shape::int::count::PointsCount;
use i_shape::int::path::IntPath;
use i_shape::int::shape::{IntShape, IntShapes};

use crate::core::fill_rule::FillRule;
use crate::core::link::OverlayLinkBuilder;
use crate::core::overlay_rule::OverlayRule;

use crate::core::solver::Solver;
use crate::segm::build::BuildSegments;
use crate::segm::segment::Segment;
use crate::vector::edge::{VectorEdge, VectorShape};

use super::graph::OverlayGraph;

/// Specifies the type of shape being processed, influencing how the shape participates in Boolean operations.
/// Note: All operations except for `Difference` are commutative, meaning the order of `Subject` and `Clip` shapes does not impact the outcome.
/// - `Subject`: The primary shape(s) for operations. Acts as the base layer in the operation.
/// - `Clip`: The modifying shape(s) that are applied to the `Subject`. Determines how the `Subject` is altered or intersected.
#[derive(Debug, Clone, Copy)]
pub enum ShapeType {
    Subject,
    Clip,
}

/// This struct is essential for describing and uploading the geometry or shapes required to construct an `OverlayGraph`. It prepares the necessary data for boolean operations.
#[derive(Clone)]
pub struct Overlay {
    pub(crate) segments: Vec<Segment>,
}

impl Overlay {
    /// Constructs a new `Overlay` instance, initializing it with a capacity that should closely match the total count of edges from all shapes being processed.
    /// This pre-allocation helps in optimizing memory usage and performance.
    /// - `capacity`: The initial capacity for storing edge data. Ideally, this should be set to the sum of the edges of all shapes to be added to the overlay, ensuring efficient data management.
    pub fn new(capacity: usize) -> Self {
        Self {
            segments: Vec::with_capacity(capacity),
        }
    }

    /// Creates a new `Overlay` instance and initializes it with subject and clip paths.
    /// - `subject_paths`: An array of paths that together define the subject shape.
    /// - `clip_paths`: An array of paths that together define the clip shape.
    pub fn with_paths(subject_paths: &[IntPath], clip_paths: &[IntPath]) -> Self {
        let mut overlay = Self::new(subject_paths.points_count() + clip_paths.points_count());
        overlay.add_paths(subject_paths, ShapeType::Subject);
        overlay.add_paths(clip_paths, ShapeType::Clip);
        overlay
    }

    /// Creates a new `Overlay` instance and initializes it with subject and clip shapes.
    /// - `subject_shapes`: An array of shapes to be used as the subject in the overlay operation.
    /// - `clip_shapes`: An array of shapes to be used as the clip in the overlay operation.
    pub fn with_shapes(subject_shapes: &[IntShape], clip_shapes: &[IntShape]) -> Self {
        let mut overlay = Self::new(subject_shapes.points_count() + clip_shapes.points_count());
        overlay.add_shapes(subject_shapes, ShapeType::Subject);
        overlay.add_shapes(clip_shapes, ShapeType::Clip);
        overlay
    }

    /// Adds a path to the overlay using an iterator, allowing for more flexible path input.
    /// This function is particularly useful when working with dynamically generated paths or
    /// when paths are not directly stored in a collection.
    /// - `iter`: An iterator over references to `IntPoint` that defines the path.
    /// - `shape_type`: Specifies the role of the added path in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
    pub fn add_path_iter<I: Iterator<Item=IntPoint>>(&mut self, iter: I, shape_type: ShapeType) {
        self.segments.append_path_iter(iter, shape_type);
    }

    /// Adds a single path to the overlay as either subject or clip paths.
    /// - `path`: A reference to a `IntPath` instance to be added.
    /// - `shape_type`: Specifies the role of the added path in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
    pub fn add_path(&mut self, path: &[IntPoint], shape_type: ShapeType) {
        self.segments.append_path_iter(path.iter().copied(), shape_type);
    }

    /// Adds multiple paths to the overlay as either subject or clip paths.
    /// - `paths`: An array of `IntPath` instances to be added to the overlay.
    /// - `shape_type`: Specifies the role of the added paths in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
    pub fn add_paths(&mut self, paths: &[IntPath], shape_type: ShapeType) {
        for path in paths.iter() {
            self.add_path(path, shape_type);
        }
    }

    /// Adds a single shape to the overlay as either a subject or clip shape.
    /// - `shape`: A reference to a `IntShape` instance to be added.
    /// - `shape_type`: Specifies the role of the added shape in the overlay operation, either as `Subject` or `Clip`.
    #[inline]
    pub fn add_shape(&mut self, shape: &IntShape, shape_type: ShapeType) {
        self.add_paths(shape, shape_type);
    }

    /// Adds multiple shapes to the overlay as either subject or clip shapes.
    /// - `shapes`: An array of `IntShape` instances to be added to the overlay.
    /// - `shape_type`: Specifies the role of the added shapes in the overlay operation, either as `Subject` or `Clip`.
    pub fn add_shapes(&mut self, shapes: &[IntShape], shape_type: ShapeType) {
        for shape in shapes.iter() {
            self.add_paths(shape, shape_type);
        }
    }

    /// Convert into vector shapes from the added paths or shapes, applying the specified fill and overlay rules. This method is particularly useful for development purposes and for creating visualizations in educational demos, where understanding the impact of different rules on the final geometry is crucial.
    /// - `fill_rule`: The fill rule to use for the shapes.
    /// - `overlay_rule`: The overlay rule to apply.
    /// - `solver`: Type of solver to use.
    pub fn into_shape_vectors(self, fill_rule: FillRule, overlay_rule: OverlayRule, solver: Solver) -> Vec<VectorShape> {
        let links = OverlayLinkBuilder::build_with_overlay_filter(self.segments, fill_rule, overlay_rule, solver);
        let graph = OverlayGraph::new(solver, links);
        graph.extract_shape_vectors(overlay_rule)
    }

    /// Convert into vectors from the added paths or shapes, applying the specified fill rule. This method is particularly useful for development purposes and for creating visualizations in educational demos, where understanding the impact of different rules on the final geometry is crucial.
    /// - `fill_rule`: The fill rule to use for the shapes.
    /// - `solver`: Type of solver to use.
    pub fn into_separate_vectors(self, fill_rule: FillRule, solver: Solver) -> Vec<VectorEdge> {
        let links = OverlayLinkBuilder::build_without_filter(self.segments, fill_rule, solver);
        OverlayGraph::new(solver, links).extract_separate_vectors()
    }

    /// Convert into `OverlayGraph` from the added paths or shapes using the specified fill rule. This graph is the foundation for executing boolean operations, allowing for the analysis and manipulation of the geometric data. The `OverlayGraph` created by this method represents a preprocessed state of the input shapes, optimized for the application of boolean operations based on the provided fill rule.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    #[inline]
    pub fn into_graph(self, fill_rule: FillRule) -> OverlayGraph {
        self.into_graph_with_solver(fill_rule, Default::default())
    }

    /// Convert into `OverlayGraph` from the added paths or shapes using the specified fill rule. This graph is the foundation for executing boolean operations, allowing for the analysis and manipulation of the geometric data. The `OverlayGraph` created by this method represents a preprocessed state of the input shapes, optimized for the application of boolean operations based on the provided fill rule.
    /// - `fill_rule`: Specifies the rule for determining filled areas within the shapes, influencing how the resulting graph represents intersections and unions.
    /// - `solver`: Type of solver to use.
    #[inline]
    pub fn into_graph_with_solver(self, fill_rule: FillRule, solver: Solver) -> OverlayGraph {
        let links = OverlayLinkBuilder::build_with_filler_filter(self.segments, fill_rule, solver);
        OverlayGraph::new(solver, links)
    }

    #[inline]
    pub fn overlay(self, overlay_rule: OverlayRule, fill_rule: FillRule) -> IntShapes {
        self.overlay_with_min_area_and_solver(overlay_rule, fill_rule, 0, Default::default())
    }

    #[inline]
    pub fn overlay_with_min_area_and_solver(self, overlay_rule: OverlayRule, fill_rule: FillRule, min_area: usize, solver: Solver) -> IntShapes {
        let links = OverlayLinkBuilder::build_with_overlay_filter(self.segments, fill_rule, overlay_rule, solver);
        let graph = OverlayGraph::new(solver, links);
        let filter = vec![false; graph.links.len()];
        graph.extract(filter, overlay_rule, min_area)
    }
}