use eframe::egui::{self, color_picker::color_edit_button_rgba, Button, Color32, RichText, Slider};

use crate::graph::Graph;

pub struct NodeEditor {}

impl NodeEditor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn name(&self) -> &'static str {
        "Node Editor"
    }

    pub fn show(&mut self, ctx: &eframe::egui::Context, open: &mut bool, graph: &mut Graph) {
        egui::Window::new("Node")
            .open(open)
            .collapsible(false)
            .show(ctx, |ui| {
                self.ui(ui, graph);
            });
    }

    fn ui(&mut self, ui: &mut egui::Ui, graph: &mut Graph) {
        if let Some(node) = graph.selected_node_mut() {
            ui.label("Node");
            ui.separator();
            ui.text_edit_singleline(&mut node.label);
            ui.separator();
            ui.add(Slider::new(&mut node.radius, 10.0..=100.0).text("Size"));
            ui.separator();

            ui.horizontal(|ui| {
                color_edit_button_rgba(ui, &mut node.color, egui::color_picker::Alpha::Opaque);
                ui.label("Color");
            });

            ui.separator();

            if ui
                .add(Button::new(RichText::new("Delete").color(Color32::WHITE)).fill(Color32::RED))
                .clicked()
            {
                graph.remove_selected_node();
            }
        } else {
            ui.label("No node selected");
        }
    }
}
