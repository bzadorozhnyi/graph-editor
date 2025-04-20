use eframe::egui::{Color32, Rgba};
use serde::{Deserialize, Serialize};

use crate::consts::{
    MAX_EDGE_LABEL_PADDING, MIN_EDGE_LABEL_PADDING, MIN_EDGE_LABEL_SIZE, MIN_EDGE_WIDTH,
    MIN_LOOP_EDGE_ANGLE,
};

use super::NodeId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EdgeId(pub usize);

#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    pub start_id: NodeId,
    pub end_id: NodeId,
    pub oriented: bool,
    pub color: Rgba,
    pub label: String,
    pub label_size: f32,
    pub padding_x: f32,
    pub padding_y: f32,
    pub width: f32,
    pub loop_rotation_angle: f32,
}

impl Edge {
    pub fn new(start_id: NodeId, end_id: NodeId) -> Self {
        Self {
            start_id,
            end_id,
            oriented: true,
            color: Rgba::from(Color32::BLACK),
            label: String::new(),
            label_size: MIN_EDGE_LABEL_SIZE,
            padding_x: (MIN_EDGE_LABEL_PADDING + MAX_EDGE_LABEL_PADDING) / 2.0,
            padding_y: (MIN_EDGE_LABEL_PADDING + MAX_EDGE_LABEL_PADDING) / 2.0,
            width: MIN_EDGE_WIDTH,
            loop_rotation_angle: MIN_LOOP_EDGE_ANGLE,
        }
    }

    pub fn is_loop(&self) -> bool {
        self.start_id == self.end_id
    }
}
