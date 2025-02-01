use eframe::egui::{Color32, Rgba};

use super::NodeId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EdgeId(pub usize);

#[derive(Debug)]
pub struct Edge {
    pub start_id: NodeId,
    pub end_id: NodeId,
    pub oriented: bool,
    pub color: Rgba,
    pub label: String,
    pub label_size: f32,
    pub padding_x: f32,
    pub padding_y: f32,
    pub width: f32
}

impl Edge {
    pub fn new(start_index: NodeId, end_index: NodeId) -> Self {
        Self {
            start_id: start_index,
            end_id: end_index,
            oriented: true,
            color: Rgba::from(Color32::BLACK),
            label: String::new(),
            label_size: 10.0,
            padding_x: 0.0,
            padding_y: 0.0,
            width: 2.0
        }
    }
}
