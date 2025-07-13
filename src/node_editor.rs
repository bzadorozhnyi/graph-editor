use eframe::egui::{
    self, color_picker::color_edit_button_rgba, Button, Color32, DragValue, Layout, RichText,
};

use crate::{
    consts::{MAX_NODE_LABEL_SIZE, MAX_NODE_SIZE, MIN_NODE_LABEL_SIZE, MIN_NODE_SIZE, UI_SPACE},
    graph::NodeShape,
    graph_workspace::GraphWorkspace,
};

pub struct NodeEditor;

impl NodeEditor {
    pub fn name(&self) -> &'static str {
        "Node Editor"
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, graph_workspace: &mut GraphWorkspace) {
        ui.vertical_centered(|ui| {
            ui.label(RichText::new(self.name()).size(24.0));
        });

        ui.separator();

        if let Some(selected_node) = graph_workspace.selected_node_mut() {
            ui.horizontal(|ui| {
                color_edit_button_rgba(
                    ui,
                    &mut selected_node.color,
                    egui::color_picker::Alpha::Opaque,
                );

                ui.add_space(UI_SPACE);

                ui.add(
                    DragValue::new(&mut selected_node.size)
                        .range(MIN_NODE_SIZE..=MAX_NODE_SIZE)
                        .speed(0.2)
                        .prefix("Size: "),
                );
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Shape:");
                ui.add_space(UI_SPACE);
                ui.selectable_value(&mut selected_node.shape, NodeShape::Circle, "⏺");
                ui.add_space(UI_SPACE);
                ui.selectable_value(&mut selected_node.shape, NodeShape::Square, "⏹");
            });

            ui.separator();

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Label: ");
                    ui.text_edit_singleline(&mut selected_node.label);
                });

                ui.add_space(UI_SPACE);

                ui.checkbox(
                    &mut selected_node.label_size_matches_node_size,
                    "Match node size",
                );

                if selected_node.label_size_matches_node_size {
                    ui.disable();
                }

                ui.add_space(UI_SPACE);

                ui.add(
                    DragValue::new(&mut selected_node.label_size)
                        .range(MIN_NODE_LABEL_SIZE..=MAX_NODE_LABEL_SIZE)
                        .speed(0.2)
                        .prefix("Label size: "),
                );
            });

            ui.separator();

            ui.with_layout(Layout::right_to_left(egui::Align::TOP), |ui| {
                if ui
                    .add(
                        Button::new(RichText::new("Delete").color(Color32::WHITE))
                            .fill(Color32::RED),
                    )
                    .clicked()
                {
                    graph_workspace.remove_selected_node();
                }
            });
        } else {
            ui.label("No node selected");
        }
    }
}
