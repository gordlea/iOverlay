use crate::bind::segment::IdSegment;
use crate::segm::segment::Segment;
use crate::segm::winding_count::WindingCount;
use crate::split::cross_solver::{CrossSolver, CrossType, EndMask};
use crate::split::fragment::Fragment;
use crate::split::grid_layout::{FragmentBuffer, GridLayout};
use crate::split::line_mark::LineMark;
use crate::split::solver::SplitSolver;

impl SplitSolver {
    pub(super) fn fragment_split<C: WindingCount>(&mut self, mut segments: Vec<Segment<C>>) -> Vec<Segment<C>> {
        let layout = if let Some(layout) = GridLayout::new(segments.iter().map(|it| it.x_segment), segments.len()) {
            layout
        } else {
            return self.tree_split(segments)
        };

        let mut buffer = FragmentBuffer::new(layout);

        let mut marks = Vec::new();
        let mut need_to_fix = true;

        let mut iter = 0;

        while need_to_fix && segments.len() > 2 {

            buffer.init_fragment_buffer(segments.iter().map(|it| it.x_segment));
            for (i, segment) in segments.iter().enumerate() {
                buffer.add_segment(i, segment.x_segment);
            }

            need_to_fix = self.process(iter, &mut buffer, &mut marks);

            if !buffer.on_border.is_empty() {
                for (&index, segments) in buffer.on_border.iter_mut() {
                    if let Some(fragments) = buffer.groups.get(index) {
                        let border_x = buffer.layout.pos(index);
                        Self::on_border_split(border_x, fragments, segments, &mut marks);
                    }
                }
            }

            if marks.is_empty() {
                return segments;
            }

            buffer.clear();

            segments = self.apply(&mut marks, segments, need_to_fix);

            marks.clear();

            iter += 1;
        }

        segments
    }

    #[inline]
    fn process(&self, iter: usize, buffer: &mut FragmentBuffer, marks: &mut Vec<LineMark>) -> bool {
        let radius = self.solver.radius(iter);
        #[cfg(feature = "allow_multithreading")]
        {
            if self.solver.multithreading.is_some() {
                return Self::parallel_split(radius, buffer, marks)
            }
        }

        Self::serial_split(radius, buffer, marks)
    }

    #[inline]
    fn serial_split(radius: i64, buffer: &mut FragmentBuffer, marks: &mut Vec<LineMark>) -> bool {
        let mut is_any_round = false;
        for group in buffer.groups.iter_mut() {
            if group.is_empty() { continue; }
            let any_round = SplitSolver::bin_split(radius, group, marks);
            is_any_round = is_any_round || any_round;
        }
        is_any_round
    }

    #[cfg(feature = "allow_multithreading")]
    fn parallel_split(radius: i64, buffer: &mut FragmentBuffer, marks: &mut Vec<LineMark>) -> bool {
        use rayon::iter::IntoParallelRefMutIterator;
        use rayon::iter::ParallelIterator;

        struct TaskResult {
            any_round: bool,
            marks: Vec<LineMark>,
        }

        let results: Vec<TaskResult> = buffer
            .groups
            .par_iter_mut()
            .map(|group| {
                let mut marks = Vec::new();
                let any_round = SplitSolver::bin_split(radius, group, &mut marks);
                TaskResult {
                    any_round,
                    marks,
                }
            })
            .collect();

        let mut is_any_round = false;
        for mut result in results.into_iter() {
            is_any_round = is_any_round || result.any_round;
            marks.append(&mut result.marks);
        }

        is_any_round
    }

    fn bin_split(radius: i64, fragments: &mut [Fragment], marks: &mut Vec<LineMark>) -> bool {
        if fragments.len() < 2 {
            return false
        }

        fragments.sort_unstable_by(|a, b| a.rect.min_y.cmp(&b.rect.min_y));

        let mut any_round = false;

        for (i, fi) in fragments.iter().enumerate().take(fragments.len() - 1) {
            for fj in fragments.iter().skip(i + 1) {
                if fi.rect.max_y < fj.rect.min_y {
                    break;
                }
                if !fi.rect.is_intersect_border_include(&fj.rect) {
                    continue;
                }

                // MARK: the intersection, ensuring the right order for deterministic results

                let is_round = if fi.x_segment < fj.x_segment {
                    SplitSolver::cross_fragments(fi, fj, marks, radius)
                } else {
                    SplitSolver::cross_fragments(fj, fi, marks, radius)
                };

                // let is_round = SplitSolver::cross(fi.index, fj.index, &fi.x_segment, &fj.x_segment, marks, radius);
                any_round = any_round || is_round
            }
        }

        any_round
    }

    fn on_border_split(border_x: i32, fragments: &[Fragment], vertical_segments: &mut [IdSegment], marks: &mut Vec<LineMark>) {
        let mut points = Vec::new();
        for fragment in fragments.iter() {
            if fragment.x_segment.b.x == border_x {
                points.push(fragment.x_segment.b)
            }
        }

        if points.is_empty() {
            return;
        }

        points.sort_unstable_by(|p0, p1| p0.y.cmp(&p1.y));
        vertical_segments.sort_by(|s0, s1| s0.x_segment.a.y.cmp(&s1.x_segment.a.y));

        let mut i = 0;
        for s in vertical_segments.iter() {
            while i < points.len() && points[i].y <= s.x_segment.a.y {
                i += 1;
            }
            let mut j = i;
            while j < points.len() && points[j].y < s.x_segment.b.y {
                marks.push(LineMark { index: s.id, point: points[j] });
                j += 1;
            }
        }
    }

    fn cross_fragments(fi: &Fragment, fj: &Fragment, marks: &mut Vec<LineMark>, radius: i64) -> bool {
        let cross = if let Some(cross) = CrossSolver::cross(&fi.x_segment, &fj.x_segment, radius) {
            cross
        } else {
            return false;
        };

        let r = radius as i32;

        match cross.cross_type {
            CrossType::Overlay => {
                let mask = CrossSolver::collinear(&fi.x_segment, &fj.x_segment);
                if mask == 0 { return false; }

                if !(fi.rect.contains_with_radius(fi.x_segment.a, r) || fj.rect.contains_with_radius(fi.x_segment.a, r)) {
                    return false;
                }

                if mask.is_target_a() {
                    marks.push(LineMark { index: fj.index, point: fi.x_segment.a });
                }

                if mask.is_target_b() {
                    marks.push(LineMark { index: fj.index, point: fi.x_segment.b });
                }

                if mask.is_other_a() {
                    marks.push(LineMark { index: fi.index, point: fj.x_segment.a });
                }

                if mask.is_other_b() {
                    marks.push(LineMark { index: fi.index, point: fj.x_segment.b });
                }
            }
            _ => {
                if !fi.rect.contains_with_radius(cross.point, r) || !fj.rect.contains_with_radius(cross.point, r) {
                    return false;
                }

                match cross.cross_type {
                    CrossType::Pure => {
                        marks.push(LineMark { index: fi.index, point: cross.point });
                        marks.push(LineMark { index: fj.index, point: cross.point });
                    }
                    CrossType::TargetEnd => {
                        marks.push(LineMark { index: fj.index, point: cross.point });
                    }
                    CrossType::OtherEnd => {
                        marks.push(LineMark { index: fi.index, point: cross.point });
                    }
                    _ => {}
                }
            }
        }

        cross.is_round
    }




}