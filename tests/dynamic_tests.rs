#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use i_float::point::IntPoint;
    use i_shape::int::path::IntPath;
    use i_shape::int::shape::IntShape;
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay::{Overlay, ShapeType};
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::core::solver::Solver;

    #[test]
    fn test_0() {
        let clip = create_star(1.0, 2.0, 7, 0.0);
        let mut r = 0.9;
        while r < 1.2 {
            let mut a = 0.0;
            while a < 2.0 * PI {
                let subj = create_star(1.0, r, 7, a);

                let mut overlay = Overlay::new(2);
                overlay.add_shape(&subj, ShapeType::Subject);
                overlay.add_shape(&clip, ShapeType::Clip);

                let graph = overlay.build_graph(FillRule::NonZero);
                let result = graph.extract_shapes(OverlayRule::Union);
                assert!(result.len() > 0);
                a += 0.005
            }
            r += 0.01
        }
    }

    #[test]
    fn test_1() {
        let mut a = 0.0;
        let clip = create_star(200.0, 30.0, 7, 0.0);
        while a < 4.0 * PI {
            let subj = create_star(200.0, 30.0, 7, a);

            let mut overlay = Overlay::new(2);
            overlay.add_shape(&subj, ShapeType::Subject);
            overlay.add_shape(&clip, ShapeType::Clip);

            let graph = overlay.build_graph(FillRule::NonZero);
            let result = graph.extract_shapes(OverlayRule::Xor);
            assert!(result.len() > 1 || result.len() == 0);
            a += 0.001
        }
    }

    #[test]
    fn test_2() {
        let mut a = 0.0;
        let clip = create_star(202.5, 33.75, 24, 0.0);
        while a < 2.0 * PI {
            let subj = create_star(202.5, 33.75, 24, a);

            let mut overlay = Overlay::new(2);
            overlay.add_shape(&subj, ShapeType::Subject);
            overlay.add_shape(&clip, ShapeType::Clip);

            let graph = overlay.build_graph(FillRule::NonZero);
            let result = graph.extract_shapes(OverlayRule::Xor);
            assert!(result.len() > 1 || result.len() == 0);
            a += 0.001
        }
    }

    #[test]
    fn test_3() {
        let mut a = 0.0;
        let clip = create_star(100.0, 10.0, 17, 0.0);
        while a < 4.0 * PI {
            let subj = create_star(100.0, 10.0, 17, a);

            let mut overlay = Overlay::new(2);
            overlay.add_shape(&subj, ShapeType::Subject);
            overlay.add_shape(&clip, ShapeType::Clip);

            let graph = overlay.build_graph(FillRule::EvenOdd);
            let result = graph.extract_shapes(OverlayRule::Xor);
            assert!(result.len() > 1 || result.len() == 0);
            a += 0.001
        }
    }

    #[test]
    fn test_4() {
        let mut a = -0.000_001;
        let clip = create_star(202.5, 33.75, 24, 0.0);
        while a < 0.000_001 {
            let subj = create_star(202.5, 33.75, 24, a);

            let mut overlay = Overlay::new(2);
            overlay.add_shape(&subj, ShapeType::Subject);
            overlay.add_shape(&clip, ShapeType::Clip);

            let graph = overlay.build_graph(FillRule::NonZero);
            let result = graph.extract_shapes(OverlayRule::Xor);
            assert!(result.len() > 1 || result.len() == 0);
            a += 0.000_000_01
        }
    }

    #[test]
    fn test_5() {
        let a = -9.9999999999999995E-7;
        let clip = create_star(202.5, 33.75, 24, 0.0);
        let subj = create_star(202.5, 33.75, 24, a);

        println!("subj {:?}", subj);

        let mut overlay = Overlay::new(2);
        overlay.add_shape(&subj, ShapeType::Subject);
        overlay.add_shape(&clip, ShapeType::Clip);

        let graph = overlay.build_graph(FillRule::NonZero);
        let result = graph.extract_shapes(OverlayRule::Xor);
        assert!(result.len() > 1 || result.len() == 0);
    }

    #[test]
    fn test_6() {
        let mut a = -0.000_001;
        let clip = create_star(100.0, 50.0, 24, 0.0);
        while a < 0.000_001 {
            let subj = create_star(100.0, 50.0, 24, a);

            let mut overlay = Overlay::new(2);
            overlay.add_shape(&subj, ShapeType::Subject);
            overlay.add_shape(&clip, ShapeType::Clip);

            let graph = overlay.build_graph(FillRule::NonZero);
            let result = graph.extract_shapes(OverlayRule::Xor);
            assert!(result.len() > 1 || result.len() == 0);
            a += 0.000_000_1
        }
    }

    #[test]
    fn test_7() {
        // let n = 10100;
        let n = 1010;
        let subj_paths = random_polygon(1000_000.0, n);

        let mut overlay = Overlay::new(n);
        overlay.add_path(&subj_paths, ShapeType::Subject);

        let graph = overlay.build_graph_with_solver(FillRule::NonZero, Solver::AUTO);
        let result = graph.extract_shapes(OverlayRule::Subject);

        assert!(!result.is_empty());
    }

    #[test]
    fn test_8() {
        let mut r = 0.004;
        while r < 1.0 {
            for n in 5..10 {
                let subj_paths = random_polygon(r, n);

                let mut overlay = Overlay::new(n);
                overlay.add_path(&subj_paths, ShapeType::Subject);

                let graph = overlay.build_graph(FillRule::NonZero);
                let result = graph.extract_shapes(OverlayRule::Subject);

                assert!(!result.is_empty());
            }
            r += 0.001;
        }
    }

    fn create_star(r0: f64, r1: f64, count: usize, angle: f64) -> IntShape {
        let da = PI / count as f64;
        let mut a = angle;

        let mut points = Vec::new();

        let sr0 = r0 * 1024.0;
        let sr1 = r1 * 1024.0;

        for _ in 0..count {
            let xr0 = (sr0 * a.cos()) as i32;
            let yr0 = (sr0 * a.sin()) as i32;

            a += da;

            let xr1 = (sr1 * a.cos()) as i32;
            let yr1 = (sr1 * a.sin()) as i32;

            a += da;

            points.push(IntPoint::new(xr0, yr0));
            points.push(IntPoint::new(xr1, yr1));
        }

        [points].to_vec()
    }

    fn random_polygon(radius: f64, n: usize) -> IntPath {
        let mut result = Vec::with_capacity(n);
        let da: f64 = PI * 0.7;
        let mut a: f64 = 0.0;
        let r = 1024.0 * radius;
        for _ in 0..n {
            let sc = a.sin_cos();

            let x = r * sc.1;
            let y = r * sc.0;

            result.push(IntPoint::new(x as i32, y as i32));
            a += da;
        }

        result
    }
}