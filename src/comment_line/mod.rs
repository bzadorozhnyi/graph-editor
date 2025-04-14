use eframe::egui::{Color32, Pos2, Rect, Rgba, Stroke};
use orientation::Orientation;

pub mod editor;
pub mod group;
pub mod orientation;

#[derive(Debug)]
pub struct CommentLine {
    pub points: Vec<Pos2>,
    pub stroke: Stroke,
}

impl CommentLine {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from(stroke: Stroke) -> Self {
        Self {
            points: vec![],
            stroke,
        }
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Orientation of a, b, c.
    fn orientation(&self, a: Pos2, b: Pos2, c: Pos2) -> Orientation {
        let value = (b.y - a.y) * (c.x - a.x) - (b.x - a.x) * (c.y - b.y);

        match value.signum() as i32 {
            0 => Orientation::Collinear,
            1 => Orientation::Clockwise,
            -1 => Orientation::CounterClockwise,
            _ => unreachable!(),
        }
    }

    /// Check if b is on (a; c) segment
    fn on_segment(&self, a: Pos2, b: Pos2, c: Pos2) -> bool {
        b.x >= a.x.min(c.x) && b.x <= a.x.max(c.x) && b.y >= a.y.min(c.y) && b.y <= a.y.max(c.y)
    }

    /// Check if segments (a; b) and (c; d) are intersecting
    fn segments_intersect(&self, a: Pos2, b: Pos2, c: Pos2, d: Pos2) -> bool {
        let o1 = self.orientation(a, b, c);
        let o2 = self.orientation(a, b, d);
        let o3 = self.orientation(c, d, a);
        let o4 = self.orientation(c, d, b);

        if o1 != o2 && o3 != o4 {
            return true;
        }

        (o1 == Orientation::Collinear && self.on_segment(a, c, b))
            || (o2 == Orientation::Collinear && self.on_segment(a, d, b))
            || (o3 == Orientation::Collinear && self.on_segment(c, a, d))
            || (o4 == Orientation::Collinear && self.on_segment(c, b, d))
    }

    /// Check if `comment_line` is intersect line
    fn is_intersect(&self, line: [Pos2; 2]) -> bool {
        let c = line[0];
        let d = line[1];

        for pair in self.points.windows(2) {
            let a = pair[0];
            let b = pair[1];

            if self.segments_intersect(a, b, c, d) {
                return true;
            }
        }

        false
    }

    /// Check if square is intersect `comment_line`
    pub fn is_intersect_square(&self, square: Rect) -> bool {
        let square_edges = [
            [square.left_top(), square.right_top()],
            [square.right_top(), square.right_bottom()],
            [square.right_bottom(), square.left_bottom()],
            [square.left_bottom(), square.left_top()],
        ];

        for point in &self.points {
            if square.contains(*point) || square_edges.iter().any(|edge| self.is_intersect(*edge)) {
                return true;
            }
        }

        false
    }
}

impl Default for CommentLine {
    fn default() -> Self {
        Self {
            points: vec![],
            stroke: Stroke::new(1.0, Rgba::from(Color32::BLACK)),
        }
    }
}
