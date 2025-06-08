use eframe::egui::{Align2, Color32, FontFamily, FontId, Painter, Pos2, Rect, Rgba, Stroke, Vec2};
use serde::{Deserialize, Serialize};

pub mod shape;
use crate::{
    consts::{
        DEFAULT_NODE_X_POSITION, DEFAULT_NODE_Y_POSITION, MIN_NODE_LABEL_SIZE, MIN_NODE_SIZE,
    },
    graph::node::shape::NodeShape,
    utils::geometry::{
        circle::rotate_cirlce_border_point,
        square::{intersect_rect_edge, rotate_square_border_point},
    },
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
            }
        }
    }

    pub fn border_point_in_direction(&self, direction: Vec2) -> Pos2 {
        match self.shape {
            NodeShape::Circle => self.position + direction.normalized() * self.size,
            NodeShape::Square => intersect_rect_edge(self.position, self.size, direction),
        }
    }

    /// Rotate a point on the border of a node around its center by a given angle (alpha, in radians).
    /// The resulting point is snapped back to the node's perimeter.
    pub fn rotate_border_point(&self, border_pos: Pos2, alpha: f32) -> Pos2 {
        match self.shape {
            NodeShape::Circle => rotate_cirlce_border_point(border_pos, self.position, alpha),
            NodeShape::Square => {
                rotate_square_border_point(border_pos, self.position, alpha, self.size)
            }
        }
    }
}
