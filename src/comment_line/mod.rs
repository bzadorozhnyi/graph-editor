use eframe::egui::{Color32, Pos2, Rgba};

pub mod editor;
pub mod group;

#[derive(Debug)]
pub struct CommentLine {
    pub points: Vec<Pos2>,
    pub color: Rgba,
    pub width: f32,
}

impl CommentLine {
    pub fn new() -> Self {
        Self {
            points: vec![],
            color: Rgba::from(Color32::BLACK),
            width: 2.0,
        }
    }

    pub fn from(color: Rgba, width: f32) -> Self {
        Self {
            points: vec![],
            color,
            width,
        }
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }
}
