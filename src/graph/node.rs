use eframe::egui::{Color32, Pos2, Rgba};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub struct NodeId(pub usize);

#[derive(Debug)]
pub struct Node {
    pub position: Pos2,
    pub radius: f32,
    pub color: Rgba,
    pub label: String,
    pub label_size_matches_node_size: bool,
    pub label_size: f32,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            position: Pos2::new(100.0, 100.0),
            radius: 20.0,
            color: Rgba::from(Color32::RED),
            label: "1".to_string(),
            label_size_matches_node_size: true,
            label_size: 20.0,
        }
    }
}

impl Node {
    pub fn new() -> Self {
        Default::default()
    }
}
