use eframe::egui;

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

            ui.label("Edge");
            ui.separator();
            ui.label(format!(
                "Start: {}",
                &graph.nodes()[&selected_edge.start_id].label
            ));
            ui.label(format!(
                "End: {}",
                &graph.nodes()[&selected_edge.end_id].label
            ));

            let selected_edge = graph.selected_edge_mut().unwrap();
            ui.checkbox(&mut selected_edge.oriented, "Oriented");
        }
    }
}
