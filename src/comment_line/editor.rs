use eframe::egui::{
    self, color_picker::color_edit_button_rgba, Color32, DragValue, Rgba,
};

pub struct CommentsEditor {
    draw_active: bool,
    erase_active: bool,
    selected_color: Rgba,
    selected_width: f32,
}

impl CommentsEditor {
    pub fn new() -> Self {
        Self {
            draw_active: false,
            erase_active: false,
            selected_color: Rgba::from(Color32::BLACK),
            selected_width: 2.0,
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

            ui.add(
                DragValue::new(&mut self.selected_width)
                    .range(1.0..=5.0)
                    .speed(0.2)
                    .prefix("Width: "),
            );

            ui.add_space(5.0);

            color_edit_button_rgba(
                ui,
                &mut self.selected_color,
                egui::color_picker::Alpha::Opaque,
            );
            ui.label("Color");

            ui.add_space(5.0);

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

    pub fn stroke_params(&self) -> (Rgba, f32) {
        (self.selected_color, self.selected_width)
    }
}
