use eframe::egui::{Color32, Pos2, Rgba};

use crate::consts::{
    DEFAULT_NODE_X_POSITION, DEFAULT_NODE_Y_POSITION, MIN_NODE_LABEL_SIZE, MIN_NODE_RADIUS,
};

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
            position: Pos2::new(DEFAULT_NODE_X_POSITION, DEFAULT_NODE_Y_POSITION),
            radius: MIN_NODE_RADIUS,
            color: Rgba::from(Color32::RED),
            label: "1".to_string(),
            label_size_matches_node_size: true,
            label_size: MIN_NODE_LABEL_SIZE,
        }
    }
}

impl Node {
    pub fn new(label: String, position: Pos2) -> Self {
        Self {
            label,
            position,
            ..Default::default()
        }
    }
}
