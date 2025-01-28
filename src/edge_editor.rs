use eframe::egui::{self, Slider};

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
            ui.checkbox(&mut selected_edge.oriented, "Oriented");

            ui.separator();
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut selected_edge.label);
                ui.label("Label");
            });

            ui.separator();
            ui.vertical(|ui| {
                ui.label("Label Size");
                ui.add(Slider::new(&mut selected_edge.label_size, 10.0..=36.0));
            });

            ui.separator();
            ui.vertical(|ui| {
                ui.label("Label Padding");
                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut selected_edge.padding_x, -40.0..=40.0).text("X"));
                    if ui.button("⟲").clicked() {
                        selected_edge.padding_x = 0.0;
                    }
                });
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut selected_edge.padding_y, -40.0..=40.0).text("Y"));
                    if ui.button("⟲").clicked() {
                        selected_edge.padding_y = 0.0;
                    }
                });
            });
        }
    }
}
