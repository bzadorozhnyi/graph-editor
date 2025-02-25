use eframe::egui::{self, Color32, Rgba, Stroke};

pub struct CommentsEditor {
    draw_active: bool,
    erase_active: bool,
    stroke: Stroke,
}

impl CommentsEditor {
    pub fn new() -> Self {
        Self {
            draw_active: false,
            erase_active: false,
            stroke: Stroke::new(1.0, Rgba::from(Color32::BLACK)),
        }
    }

    pub fn name(&self) -> &'static str {
        "Comments Editor"
    }

    pub fn show(&mut self, ctx: &eframe::egui::Context, open: &mut bool) {
        egui::Window::new("Comments")
            .open(open)
            .collapsible(false)
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.toggle_value(&mut self.draw_active, "✏").clicked() {
                if self.draw_active && self.erase_active {
                    self.erase_active = false;
                }
            }

            ui.add_space(5.0);

            ui.add(&mut self.stroke);

            if ui.toggle_value(&mut self.erase_active, "🗑").clicked() {
                if self.draw_active && self.erase_active {
                    self.draw_active = false;
                }
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
