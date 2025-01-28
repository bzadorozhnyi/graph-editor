use eframe::egui::{Color32, Pos2, Rgba};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct NodeId(pub usize);

#[derive(Debug)]
pub struct Node {
    pub position: Pos2,
    pub radius: f32,
    pub color: Rgba,
    pub label: String,
}

impl Node {
    pub fn new() -> Self {
        Self {
            position: Pos2::new(100.0, 100.0),
            radius: 20.0,
            color: Rgba::from(Color32::RED),
            label: "1".to_string(),
        }
    }
}
