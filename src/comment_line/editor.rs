use eframe::egui::{self, Button, Color32, Layout, Rgba, RichText, Stroke};

use crate::consts::UI_SPACE;

use super::group::CommentsGroup;

pub struct CommentsEditor {
    draw_active: bool,
    erase_active: bool,
    stroke: Stroke,
}

impl Default for CommentsEditor {
    fn default() -> Self {
        Self {
            draw_active: false,
            erase_active: false,
            stroke: Stroke::new(1.0, Rgba::from(Color32::BLACK)),
        }
    }
}

impl CommentsEditor {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn name(&self) -> &'static str {
        "Comments Editor"
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, comment_lines: &mut CommentsGroup) {
        ui.vertical_centered(|ui| {
            ui.label(RichText::new(self.name()).size(24.0));
        });

        ui.add_space(UI_SPACE);

        ui.separator();

        ui.add_space(UI_SPACE);

        ui.horizontal(|ui| {
            if ui.toggle_value(&mut self.draw_active, "✏").clicked()
                && self.draw_active
                && self.erase_active
            {
                self.erase_active = false;
            }

            ui.add_space(UI_SPACE);

            ui.add(&mut self.stroke);

            if ui.toggle_value(&mut self.erase_active, "🗑").clicked()
                && self.draw_active
                && self.erase_active
            {
                self.draw_active = false;
            }
        });

        ui.add_space(UI_SPACE);

        ui.separator();

        ui.add_space(UI_SPACE);

        ui.with_layout(Layout::right_to_left(egui::Align::TOP), |ui| {
            if ui
                .add(
                    Button::new(RichText::new("Clear all").color(Color32::WHITE))
                        .fill(Color32::RED),
                )
                .clicked()
            {
                comment_lines.clear();
            }
        });
    }

    pub fn draw_mode_active(&self) -> bool {
        self.draw_active
    }

    pub fn erase_mode_active(&self) -> bool {
        self.erase_active
    }

    pub fn selected_stroke(&self) -> Stroke {
        self.stroke
    }
}
