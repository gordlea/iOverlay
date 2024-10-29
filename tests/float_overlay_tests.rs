#[cfg(test)]
mod tests {
    use i_float::adapter::FloatPointAdapter;
    use i_float::float_point::{FloatPoint, FloatPointCompatible};
    use i_float::float_rect::FloatRect;
    use rand::Rng;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::ShapeType;
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::float::overlay::FloatOverlay;
    use i_overlay::float::string_overlay::FloatStringOverlay;
    use i_overlay::string::rule::StringRule;
    use i_overlay::string::clip::ClipRule;

    #[derive(Clone)]
    struct FPoint {
        x: f32,
        y: f32,
    }

    impl FPoint {
        fn new(x: f32, y: f32) -> Self {
            Self { x, y }
        }
    }

    impl FloatPointCompatible<f32> for FPoint {
        #[inline(always)]
        fn from_float_point(float_point: FloatPoint<f32>) -> Self {
            Self { x: float_point.x, y: float_point.y }
        }

        #[inline(always)]
        fn to_float_point(&self) -> FloatPoint<f32> {
            FloatPoint::new(self.x, self.y)
        }
    }


    #[test]
    fn test_00() {
        let shape_0 = [
            [
                FPoint::new(0.0, 0.0),
                FPoint::new(0.0, 1.0),
                FPoint::new(1.0, 1.0),
                FPoint::new(1.0, 0.0)
            ].to_vec()
        ];
        let shape_1 = [
            [
                FPoint::new(1.0, 0.0),
                FPoint::new(1.0, 1.0),
                FPoint::new(2.0, 1.0),
                FPoint::new(2.0, 0.0)
            ].to_vec()
        ];

        let union = FloatOverlay::with_paths(&shape_0, &shape_1)
            .into_graph(FillRule::NonZero)
            .extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }

    #[test]
    fn test_01() {
        let a = (1 << 30) as f32;

        let shape_0 = [
            [
                FPoint::new(0.0, 0.0),
                FPoint::new(0.0, a),
                FPoint::new(a, a),
                FPoint::new(a, 0.0)
            ].to_vec()
        ];
        let shape_1 = [
            [
                FPoint::new(a, 0.0),
                FPoint::new(a, a),
                FPoint::new(2.0 * a, a),
                FPoint::new(2.0 * a, 0.0)
            ].to_vec()
        ];

        let union = FloatOverlay::with_paths(&shape_0, &shape_1)
            .into_graph(FillRule::NonZero)
            .extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }

    #[test]
    fn test_02() {
        let i: usize = 1 << 48;
        let a = i as f32;

        let shape_0 = [
            [
                FPoint::new(0.0, 0.0),
                FPoint::new(0.0, a),
                FPoint::new(a, a),
                FPoint::new(a, 0.0)
            ].to_vec()
        ];
        let shape_1 = [
            [
                FPoint::new(a, 0.0),
                FPoint::new(a, a),
                FPoint::new(2.0 * a, a),
                FPoint::new(2.0 * a, 0.0)
            ].to_vec()
        ];

        let union = FloatOverlay::with_paths(&shape_0, &shape_1)
            .into_graph(FillRule::NonZero)
            .extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }

    #[test]
    fn test_03() {
        let i: usize = 1 << 48;
        let a = 1.0 / i as f32;

        let shape_0 = [
            [
                FPoint::new(0.0, 0.0),
                FPoint::new(0.0, a),
                FPoint::new(a, a),
                FPoint::new(a, 0.0)
            ].to_vec()
        ];
        let shape_1 = [
            [
                FPoint::new(a, 0.0),
                FPoint::new(a, a),
                FPoint::new(2.0 * a, a),
                FPoint::new(2.0 * a, 0.0)
            ].to_vec()
        ];

        let union = FloatOverlay::with_paths(&shape_0, &shape_1)
            .into_graph(FillRule::NonZero)
            .extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }

    #[test]
    fn test_04() {
        let a = 0.9;

        let shape_0 = [
            [
                FPoint::new(0.0, 0.0),
                FPoint::new(0.0, a),
                FPoint::new(a, a),
                FPoint::new(a, 0.0)
            ].to_vec()
        ];
        let shape_1 = [
            [
                FPoint::new(a, 0.0),
                FPoint::new(a, a),
                FPoint::new(2.0 * a, a),
                FPoint::new(2.0 * a, 0.0)
            ].to_vec()
        ];

        let union = FloatOverlay::with_paths(&shape_0, &shape_1)
            .into_graph(FillRule::NonZero)
            .extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }

    #[test]
    fn test_05() {
        let a = 0.99999_99999_99999_9;

        let shape_0 = [
            [
                FPoint::new(0.0, 0.0),
                FPoint::new(0.0, a),
                FPoint::new(a, a),
                FPoint::new(a, 0.0)
            ].to_vec()
        ];
        let shape_1 = [
            [
                FPoint::new(a, 0.0),
                FPoint::new(a, a),
                FPoint::new(2.0 * a, a),
                FPoint::new(2.0 * a, 0.0)
            ].to_vec()
        ];

        let union = FloatOverlay::with_paths(&shape_0, &shape_1)
            .into_graph(FillRule::NonZero)
            .extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }

    #[test]
    fn test_06() {
        let a = 1.99999_99999_99999;

        let shape_0 = [
            [
                FPoint::new(0.0, 0.0),
                FPoint::new(0.0, a),
                FPoint::new(a, a),
                FPoint::new(a, 0.0)
            ].to_vec()
        ];
        let shape_1 = [
            [
                FPoint::new(a, 0.0),
                FPoint::new(a, a),
                FPoint::new(2.0 * a, a),
                FPoint::new(2.0 * a, 0.0)
            ].to_vec()
        ];

        let union = FloatOverlay::with_paths(&shape_0, &shape_1)
            .into_graph(FillRule::NonZero)
            .extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 4);
    }

    #[test]
    fn test_07() {
        let shape_0 = [
            [
                FPoint::new(-10.0, -10.0),
                FPoint::new(-10.0, 10.0),
                FPoint::new(10.0, 10.0),
                FPoint::new(10.0, -10.0)
            ].to_vec()
        ];
        let shape_1 = [
            [
                FPoint::new(-5.0, -5.0),
                FPoint::new(-5.0, 15.0),
                FPoint::new(15.0, 15.0),
                FPoint::new(15.0, -5.0),
            ].to_vec()
        ];

        let union = FloatOverlay::with_paths(&shape_0, &shape_1)
            .into_graph(FillRule::NonZero)
            .extract_shapes(OverlayRule::Union);

        assert_eq!(union.len(), 1);
        assert_eq!(union[0].len(), 1);
        assert_eq!(union[0][0].len(), 8);
    }

    #[test]
    fn test_random() {
        let mut rng = rand::thread_rng();
        for n in 5..=10 {
            let mut points = vec![FPoint::new(0.0, 0.0); n];
            for _ in 0..=1000 {
                for i in 0..n {
                    let x = rng.gen_range(-1.0..=1.0);
                    let y = rng.gen_range(-1.0..=1.0);
                    points[i] = FPoint::new(x, y);
                }
            }
        }
    }

    #[test]
    fn test_empty_0() {
        let path = [
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
        ];

        let shapes = FloatOverlay::new(FloatPointAdapter::with_iter(path.iter()), path.len())
            .into_graph(FillRule::NonZero)
            .extract_shapes(OverlayRule::Subject);

        assert_eq!(shapes.is_empty(), true);
    }

    #[test]
    fn test_empty_1() {
        let shape = [
            [
                FPoint::new(-10.0, -10.0),
                FPoint::new(-10.0, 10.0),
                FPoint::new(10.0, 10.0),
                FPoint::new(10.0, -10.0)
            ].to_vec()
        ];

        let mut overlay = FloatOverlay::new(FloatPointAdapter::with_iter(shape.iter().flatten()), shape.len());
        overlay.add_paths(shape, ShapeType::Subject);

        let graph = overlay.into_graph(FillRule::NonZero);
        let shapes = graph.extract_shapes(OverlayRule::Subject);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_empty_2() {
        let shape_0 = [
            [
                FPoint::new(-10.0, -10.0),
                FPoint::new(-10.0, 10.0),
                FPoint::new(10.0, 10.0),
                FPoint::new(10.0, -10.0)
            ].to_vec()
        ];

        let shape_1 = [
            [
                FPoint::new(-500.0, -500.0)
            ].to_vec()
        ];


        let shapes = FloatOverlay::with_paths(&shape_0, &shape_1)
            .into_graph(FillRule::NonZero)
            .extract_shapes(OverlayRule::Subject);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_empty_3() {
        let shape_0 = [
            [
                FPoint::new(-10.0, -10.0),
                FPoint::new(-10.0, 10.0),
                FPoint::new(10.0, 10.0),
                FPoint::new(10.0, -10.0)
            ].to_vec()
        ].to_vec();

        let shape_1 = [
            [
                FPoint::new(-500.0, -500.0),
                FPoint::new(-500.0, 500.0)
            ].to_vec()
        ].to_vec();

        let shapes = FloatOverlay::with_paths(&shape_0, &shape_1)
            .into_graph(FillRule::NonZero)
            .extract_shapes(OverlayRule::Subject);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].len(), 1);
        assert_eq!(shapes[0][0].len(), 4);
    }

    #[test]
    fn test_empty_4() {
        let path = [FPoint::new(0.0, 0.0)];
        let shapes = FloatOverlay::new(FloatPointAdapter::with_iter(path.iter()), path.len())
            .unsafe_add_path(&path, ShapeType::Subject)
            .into_graph(FillRule::NonZero)
            .extract_shapes(OverlayRule::Subject);

        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_empty_5() {
        let shapes = FloatOverlay::with_path(&[FPoint::new(0.0, 0.0)], &[FPoint::new(1.0, 0.0)])
            .into_graph(FillRule::NonZero)
            .extract_shapes(OverlayRule::Subject);

        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_empty_6() {
        let path = [FPoint::new(0.0, 0.0), FPoint::new(1.0, 0.0)];
        let shapes = FloatOverlay::new(FloatPointAdapter::with_iter(path.iter()), path.len())
            .unsafe_add_path(&path, ShapeType::Subject)
            .into_graph(FillRule::NonZero)
            .extract_shapes(OverlayRule::Subject);

        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_slice_0() {
        let rect = FloatRect::new(-10.0, 10.0, -15.0, 15.0);
        let shapes = FloatStringOverlay::new(FloatPointAdapter::new(rect), 5)
            .unsafe_add_path(&[
                FPoint::new(-10.0, -10.0),
                FPoint::new(-10.0, 10.0),
                FPoint::new(10.0, 10.0),
                FPoint::new(10.0, -10.0),
            ])
            .unsafe_add_string_line(&[FPoint::new(0.0, -15.0), FPoint::new(0.0, 15.0)])
            .into_graph(FillRule::NonZero)
            .extract_shapes(StringRule::Slice);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_slice_1() {
        let rect = FloatRect::new(-10.0, 10.0, -10.0, 10.0);
        let shapes = FloatStringOverlay::new(FloatPointAdapter::new(rect), 5)
            .unsafe_add_path(&[
                FPoint::new(-10.0, -10.0),
                FPoint::new(-10.0, 10.0),
                FPoint::new(10.0, 10.0),
                FPoint::new(10.0, -10.0),
            ])
            .unsafe_add_string_line(&[FPoint::new(0.0, -5.0), FPoint::new(0.0, 5.0)])
            .into_graph(FillRule::NonZero)
            .extract_shapes(StringRule::Slice);

        assert_eq!(shapes.len(), 1);
    }

    #[test]
    fn test_slice_2() {
        let path_0 = [
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ];

        let path_1 = [
            FPoint::new(-15.0, -15.0),
            FPoint::new(0.0, 0.0),
            FPoint::new(-15.0, 15.0)
        ];

        let shapes = FloatStringOverlay::new(
            FloatPointAdapter::with_iter(path_0.iter().chain(path_1.iter())),
            6,
        )
            .unsafe_add_path(&path_0)
            .unsafe_add_string_path(&path_1, true)
            .into_graph(FillRule::NonZero)
            .extract_shapes(StringRule::Slice);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_slice_3() {
        let path_0 = [
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ];

        let path_1 = [
            FPoint::new(0.0, -5.0),
            FPoint::new(0.0, 5.0),
            FPoint::new(15.0, 5.0),
            FPoint::new(15.0, -5.0),
        ];

        let shapes = FloatStringOverlay::new(
            FloatPointAdapter::with_iter(path_0.iter().chain(path_1.iter())),
            6,
        )
            .unsafe_add_path(&path_0)
            .unsafe_add_string_path(&path_1, false)
            .into_graph(FillRule::NonZero)
            .extract_shapes(StringRule::Slice);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_slice_4() {
        let path_0 = [
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ];

        let path_1 = [
            FPoint::new(-5.0, -5.0),
            FPoint::new(-5.0, 5.0),
            FPoint::new(5.0, 5.0),
            FPoint::new(5.0, -5.0),
        ];

        let shapes = FloatStringOverlay::new(
            FloatPointAdapter::with_iter(path_0.iter().chain(path_1.iter())),
            6,
        )
            .unsafe_add_path(&path_0)
            .unsafe_add_string_path(&path_1, false)
            .into_graph(FillRule::NonZero)
            .extract_shapes(StringRule::Slice);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_ext_slice_0() {
        let shapes = [
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ].slice_by_line([FPoint::new(0.0, -15.0), FPoint::new(0.0, 15.0)], FillRule::NonZero);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_ext_slice_1() {
        let shapes = [
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ].slice_by_line([FPoint::new(0.0, -5.0), FPoint::new(0.0, 5.0)], FillRule::NonZero);

        assert_eq!(shapes.len(), 1);
    }

    #[test]
    fn test_ext_slice_2() {
        let shapes = [
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ].slice_by_path(&vec![
            FPoint::new(-15.0, -15.0),
            FPoint::new(0.0, 0.0),
            FPoint::new(-15.0, 15.0),
        ], true, FillRule::NonZero);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_ext_slice_3() {
        let shapes = [
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ].slice_by_path(&vec![
            FPoint::new(0.0, -5.0),
            FPoint::new(0.0, 5.0),
            FPoint::new(15.0, 5.0),
            FPoint::new(15.0, -5.0),
        ], false, FillRule::NonZero);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_ext_slice_4() {
        let shapes = [
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ].slice_by_path(&vec![
            FPoint::new(-5.0, -5.0),
            FPoint::new(-5.0, 5.0),
            FPoint::new(5.0, 5.0),
            FPoint::new(5.0, -5.0),
        ], false, FillRule::NonZero);

        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_clip_empty_path() {
        let path: F32Path = vec![];
        let result_0 = path.clip_line(
            [FPoint::new(0.0, 0.0), FPoint::new(1.0, 0.0)],
            FillRule::NonZero,
            ClipRule { invert: false, boundary_included: false },
        );

        let result_1 = path.clip_line(
            [FPoint::new(0.0, 0.0), FPoint::new(1.0, 0.0)],
            FillRule::NonZero,
            ClipRule { invert: true, boundary_included: false },
        );

        assert!(result_0.is_empty());
        assert_eq!(result_1.len(), 1);
    }

    #[test]
    fn test_clip_simple() {
        let path: F32Path = vec![
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ];
        let result_0 = path.clip_line(
            [FPoint::new(0.0, -15.0), FPoint::new(0.0, 15.0)],
            FillRule::NonZero,
            ClipRule { invert: false, boundary_included: false },
        );

        let result_1 = path.clip_line(
            [FPoint::new(0.0, -15.0), FPoint::new(0.0, 15.0)],
            FillRule::NonZero,
            ClipRule { invert: true, boundary_included: false },
        );

        assert_eq!(result_0.len(), 1);
        assert_eq!(result_1.len(), 2);
    }

    #[test]
    fn test_clip_boundary() {
        let path: F32Path = vec![
            FPoint::new(-10.0, -10.0),
            FPoint::new(-10.0, 10.0),
            FPoint::new(10.0, 10.0),
            FPoint::new(10.0, -10.0),
        ];
        let result_0 = path.clip_line(
            [FPoint::new(-10.0, -15.0), FPoint::new(-10.0, 15.0)],
            FillRule::NonZero,
            ClipRule { invert: false, boundary_included: false },
        );

        let result_1 = path.clip_line(
            [FPoint::new(-10.0, -15.0), FPoint::new(-10.0, 15.0)],
            FillRule::NonZero,
            ClipRule { invert: false, boundary_included: true },
        );

        assert_eq!(result_0.len(), 0);
        assert_eq!(result_1.len(), 1);
    }
}