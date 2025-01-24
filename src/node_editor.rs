use eframe::egui::{self, color_picker::color_edit_button_rgba, Slider};

use crate::graph::Node;

pub struct NodeEditor {}

impl NodeEditor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn name(&self) -> &'static str {
        "Node Editor"
    }

    pub fn show(
        &mut self,
        ctx: &eframe::egui::Context,
        open: &mut bool,
        selected_node: Option<&mut Node>,
    ) {
        egui::Window::new("Node")
            .open(open)
            .collapsible(false)
            .show(ctx, |ui| {
                self.ui(ui, selected_node);
            });
    }

    fn ui(&mut self, ui: &mut egui::Ui, selected_node: Option<&mut Node>) {
        ui.add_enabled_ui(selected_node.is_some(), |ui| {
            if let Some(node) = selected_node {
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
            } else {
                ui.label("No node selected");
            }
        });
    }
}
