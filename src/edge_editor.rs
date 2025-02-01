use eframe::egui::{
    self, color_picker::color_edit_button_rgba, Button, Color32, DragValue, RichText,
};

use crate::graph::Graph;

pub struct EdgeEditor {}

impl EdgeEditor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn name(&self) -> &'static str {
        "Edge Editor"
    }

    pub fn show(&mut self, ctx: &eframe::egui::Context, open: &mut bool, graph: &mut Graph) {
        egui::Window::new("Edge")
            .open(open)
            .collapsible(false)
            .show(ctx, |ui| {
                self.ui(ui, graph);
            });
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, graph: &mut Graph) {
        if graph.selected_edge_id().is_none() {
            ui.label("No edge selected");
        } else {
            let selected_edge = graph.selected_edge().unwrap();

            ui.vertical(|ui| {
                ui.label(format!(
                    "{} ➡ {}",
                    &graph.nodes()[&selected_edge.start_id].label,
                    &graph.nodes()[&selected_edge.end_id].label
                ));
            });
            ui.separator();

            let selected_edge = graph.selected_edge_mut().unwrap();
            ui.horizontal(|ui| {
                color_edit_button_rgba(
                    ui,
                    &mut selected_edge.color,
                    egui::color_picker::Alpha::Opaque,
                );
                let oriented = if selected_edge.oriented {
                    "Oriented"
                } else {
                    "Unoriented"
                };
                ui.toggle_value(&mut selected_edge.oriented, oriented);
            });

            ui.add_space(5.0);
            ui.add(
                DragValue::new(&mut selected_edge.width)
                    .range(1.0..=5.0)
                    .speed(0.2)
                    .prefix("Width: "),
            );

            ui.separator();
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut selected_edge.label);
                ui.label("Label");
            });

            ui.separator();
            ui.vertical(|ui| {
                ui.label("Label");
                ui.horizontal(|ui| {
                    ui.add(
                        DragValue::new(&mut selected_edge.label_size)
                            .range(10.0..=36.0)
                            .speed(0.2)
                            .prefix("Size: "),
                    );
                    if ui.button("⟲").clicked() {
                        selected_edge.label_size = 10.0;
                    }

                    ui.add_space(10.0);

                    ui.add(
                        DragValue::new(&mut selected_edge.padding_x)
                            .range(-100.0..=100.0)
                            .speed(1.0)
                            .prefix("X: "),
                    );
                    if ui.button("⟲").clicked() {
                        selected_edge.padding_x = 0.0;
                    }

                    ui.add_space(10.0);

                    ui.add(
                        DragValue::new(&mut selected_edge.padding_y)
                            .range(-100.0..=100.0)
                            .speed(1.0)
                            .prefix("Y: "),
                    );
                    if ui.button("⟲").clicked() {
                        selected_edge.padding_y = 0.0;
                    }
                });
            });

            ui.separator();

            if ui
                .add(Button::new(RichText::new("Delete").color(Color32::WHITE)).fill(Color32::RED))
                .clicked()
            {
                graph.remove_selected_edge();
            }
        }
    }
}
