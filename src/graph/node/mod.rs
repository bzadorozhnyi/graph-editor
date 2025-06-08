use eframe::egui::{Align2, Color32, FontFamily, FontId, Painter, Pos2, Rect, Rgba, Stroke, Vec2};
use serde::{Deserialize, Serialize};

pub mod shape;
use crate::{
    consts::{
        DEFAULT_NODE_X_POSITION, DEFAULT_NODE_Y_POSITION, MIN_NODE_LABEL_SIZE, MIN_NODE_SIZE,
    }, graph::node::shape::NodeShape,
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub struct NodeId(pub usize);

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub position: Pos2,
    pub size: f32,
    pub shape: NodeShape,
    pub color: Rgba,
    pub label: String,
    pub label_size_matches_node_size: bool,
    pub label_size: f32,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            position: Pos2::new(DEFAULT_NODE_X_POSITION, DEFAULT_NODE_Y_POSITION),
            size: MIN_NODE_SIZE,
            shape: NodeShape::Circle,
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

    fn draw_shape(&self, painter: &Painter) {
        match self.shape {
            NodeShape::Circle => painter.circle(self.position, self.size, self.color, Stroke::NONE),
            NodeShape::Square => painter.rect(
                Rect::from_center_size(self.position, Vec2::splat(2.0 * self.size)),
                2.0,
                self.color,
                Stroke::NONE,
                eframe::egui::StrokeKind::Inside,
            ),
        };
    }

    pub fn draw(&self, painter: &Painter) {
        self.draw_shape(painter);

        let label_size = if self.label_size_matches_node_size {
            self.size
        } else {
            self.label_size
        };

        painter.text(
            self.position,
            Align2::CENTER_CENTER,
            &self.label,
            FontId::new(label_size, FontFamily::Monospace),
            Color32::BLACK,
        );
    }

    pub fn is_clicked(&self, pointer_pos: Pos2) -> bool {
        match self.shape {
            NodeShape::Circle => self.position.distance(pointer_pos) < self.size,
            NodeShape::Square => {
                let rect = Rect::from_center_size(self.position, Vec2::splat(2.0 * self.size));
                rect.contains(pointer_pos)
            },
        }
    }
}
