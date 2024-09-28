# iOverlay

[![crates.io version](https://img.shields.io/crates/v/i_overlay.svg)](https://crates.io/crates/i_overlay)
[![docs.rs docs](https://docs.rs/i_overlay/badge.svg)](https://docs.rs/i_overlay)

![Balloons](readme/balloons.svg)

The iOverlay library provides high-performance boolean operations on polygons, including union, intersection, difference, and xor. It is designed for applications that require precise polygon operations, such as computer graphics, CAD systems, and geographical information systems (GIS). By supporting both integer (i32) and floating-point (f32, f64) APIs, iOverlay offers flexibility and precision across diverse use cases.  

*For detailed performance benchmarks, check out the* [Performance Comparison](https://ishape-rust.github.io/iShape-js/overlay/performance/performance.html)


## [Documentation](https://ishape-rust.github.io/iShape-js/overlay/stars_demo.html)
Try out iOverlay with an interactive demo:

- [Stars Rotation](https://ishape-rust.github.io/iShape-js/overlay/stars_demo.html)
- [Shapes Editor](https://ishape-rust.github.io/iShape-js/overlay/shapes_editor.html)



## Features

- **Operations**: union, intersection, difference, and exclusion.
- **Polygons**: with holes, self-intersections, and multiple paths.
- **Simplification**: removes degenerate vertices and merges collinear edges.
- **Fill Rules**: even-odd and non-zero.
- **Data Types**: Supports i32, f32, and f64 APIs.

## Getting Started

Add the following to your Cargo.toml:
```
[dependencies]
i_overlay = "^1.6"
```

### Hello world

Let's union two squares

### f64 Example ###

```rust
let subj = [
    // Define the subject polygon (a square)
    F64Point::new(-10.0, -10.0),
    F64Point::new(-10.0, 10.0),
    F64Point::new(10.0, 10.0),
    F64Point::new(10.0, -10.0),
].to_vec();

let clip = [
    // Define the clip polygon (a slightly shifted square)
    F64Point::new(-5.0, -5.0),
    F64Point::new(-5.0, 15.0),
    F64Point::new(15.0, 15.0),
    F64Point::new(15.0, -5.0),
].to_vec();

let mut overlay = F64Overlay::new();

overlay.add_path(subj, ShapeType::Subject);
overlay.add_path(clip, ShapeType::Clip);

let graph = overlay.into_graph(FillRule::NonZero);
let shapes = graph.extract_shapes(OverlayRule::Union);

println!("shapes count: {}", shapes.len());

if shapes.len() > 0 {
    let contour = &shapes[0][0];
    println!("shape 0 contour: ");
    for p in contour {
        let x = p.x;
        let y = p.y;
        println!("({}, {})", x, y);
    }
}
```
The result of the `extract_shapes` function for `f64` returns a `Vec<F64Shapes>`:

- `Vec<F64Shape>`: A collection of shapes.
- `F64Shape`: Represents one shape, consisting of:
  - `Vec<F64Path>`: A list of paths (contours).
  - The first path is the outer boundary (clockwise), and subsequent paths represent holes (counterclockwise).
- `F64Path`: A series of points (`Vec<F64Point>`) forming a closed contour.

**Note**: _Outer boundary paths have a clockwise order, and holes have a counterclockwise order. [More information](https://ishape-rust.github.io/iShape-js/overlay/contours/contours.html) about contours._


### i32 Example ###

```rust
let subj = [
    // Define the subject polygon (a square)
    IntPoint::new(-10, -10),
    IntPoint::new(-10, 10),
    IntPoint::new(10, 10),
    IntPoint::new(10, -10),
].to_vec();

let clip = [
    // Define the clip polygon (a slightly shifted square)
    IntPoint::new(-5, -5),
    IntPoint::new(-5, 15),
    IntPoint::new(15, 15),
    IntPoint::new(15, -5),
].to_vec();

let shapes = Overlay::with_paths(&[subj], &[clip])
    .into_graph(FillRule::NonZero)
    .extract_shapes(OverlayRule::Union);

println!("shapes count: {}", shapes.len());

if shapes.len() > 0 {
    let contour = &shapes[0][0];
    println!("shape 0 contour: ");
    for p in contour {
        let x = p.x;
        let y = p.y;
        println!("({}, {})", x, y);
    }
}
```
The `extract_shapes` function for `i32` returns a `Vec<IntShapes>`:

- `Vec<IntShape>`: A collection of shapes.
- `IntShape`: Represents a shape made up of:
  - `Vec<IntPath>`: A list of paths (contours).
  - The first path is the outer boundary (clockwise), and subsequent paths represent holes (counterclockwise).
- `IntPath`: A sequence of points (`Vec<IntPoint>`) forming a closed contour.

**Note**: _Outer boundary paths have a clockwise order, and holes have a counterclockwise order. [More information](https://ishape-rust.github.io/iShape-js/overlay/contours/contours.html) about contours._

# Overlay Rules

<img src="readme/ab.svg" alt="AB" style="width:50%;">

### Union, A or B
<img src="readme/union.svg" alt="Union" style="width:50%;">

### Intersection, A and B
<img src="readme/intersection.svg" alt="Intersection" style="width:50%;">

### Difference, A - B
<img src="readme/difference_ab.svg" alt="Difference" style="width:50%;">

### Inverse Difference, B - A
<img src="readme/difference_ba.svg" alt="Inverse Difference" style="width:50%;">

### Exclusion, A xor B
<img src="readme/exclusion.svg" alt="Exclusion" style="width:50%;">
