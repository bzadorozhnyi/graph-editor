use eframe::egui::{Align2, Color32, FontFamily, FontId, Painter, Pos2, Rgba, Stroke};

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

    pub fn draw(&self, painter: &Painter) {
        painter.circle(
            self.position,
            self.radius,
            self.color,
            Stroke::new(0.0, Color32::BLACK),
        );

        painter.text(
            self.position,
            Align2::CENTER_CENTER,
            self.label.clone(),
            FontId::new(self.radius, FontFamily::Monospace),
            Color32::BLACK,
        );
    }
}
