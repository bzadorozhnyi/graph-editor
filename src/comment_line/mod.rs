use eframe::egui::{Color32, Pos2, Rgba, Stroke};

pub mod editor;
pub mod group;

#[derive(Debug)]
pub struct CommentLine {
    pub points: Vec<Pos2>,
    pub stroke: Stroke
}

impl CommentLine {
    pub fn new() -> Self {
        Self {
            points: vec![],
            stroke: Stroke::new(1.0, Rgba::from(Color32::BLACK))
        }
    }

    pub fn from(stroke: Stroke) -> Self {
        Self {
            points: vec![],
            stroke
        }
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }
}
