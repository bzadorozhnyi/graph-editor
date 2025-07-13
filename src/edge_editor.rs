use eframe::egui::{
    self, color_picker::color_edit_button_rgba, Button, Color32, DragValue, Layout, RichText,
};

use crate::{
    consts::{
        MAX_EDGE_LABEL_PADDING, MAX_EDGE_LABEL_SIZE, MAX_EDGE_WIDTH, MAX_LOOP_EDGE_ANGLE,
        MIN_EDGE_LABEL_PADDING, MIN_EDGE_LABEL_SIZE, MIN_EDGE_WIDTH, MIN_LOOP_EDGE_ANGLE, UI_SPACE,
    },
    graph_workspace::GraphWorkspace,
};

pub struct EdgeEditor;

impl EdgeEditor {
    pub fn name(&self) -> &'static str {
        "Edge Editor"
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, graph_workspace: &mut GraphWorkspace) {
        ui.vertical_centered(|ui| {
            ui.label(RichText::new(self.name()).size(24.0));
        });

        ui.separator();

        if graph_workspace.selected_edge_id().is_none() {
            ui.label("No edge selected");
        } else {
            let selected_edge = graph_workspace.selected_edge().unwrap();

            ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
                ui.label(format!(
                    "{} ➡ {}",
                    &graph_workspace.node(&selected_edge.start_id).unwrap().label,
                    &graph_workspace.node(&selected_edge.end_id).unwrap().label
                ));
            });
            ui.separator();

            let selected_edge = graph_workspace.selected_edge_mut().unwrap();
            ui.horizontal(|ui| {
                color_edit_button_rgba(
                    ui,
                    &mut selected_edge.color,
                    egui::color_picker::Alpha::Opaque,
                );
                ui.add_space(UI_SPACE);
                ui.add(
                    DragValue::new(&mut selected_edge.width)
                        .range(MIN_EDGE_WIDTH..=MAX_EDGE_WIDTH)
                        .speed(0.2)
                        .prefix("Width: "),
                );

                // loop direction is unnecessary
                if !selected_edge.is_loop() {
                    ui.add_space(UI_SPACE);
                    let oriented = if selected_edge.oriented {
                        "Oriented"
                    } else {
                        "Unoriented"
                    };
                    ui.toggle_value(&mut selected_edge.oriented, oriented);
                }
            });

            if selected_edge.is_loop() {
                ui.separator();
                ui.add(
                    DragValue::new(&mut selected_edge.loop_rotation_angle)
                        .range(MIN_LOOP_EDGE_ANGLE..=MAX_LOOP_EDGE_ANGLE)
                        .speed(1)
                        .prefix("Loop rot. angle: ")
                        .suffix("°"),
                );
            }

            ui.separator();
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Label: ");
                    ui.text_edit_singleline(&mut selected_edge.label);
                });

                ui.add_space(UI_SPACE);

                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.add(
                            DragValue::new(&mut selected_edge.label_size)
                                .range(MIN_EDGE_LABEL_SIZE..=MAX_EDGE_LABEL_SIZE)
                                .speed(0.2)
                                .prefix("Size: "),
                        );
                        if ui.button("⟲").clicked() {
                            selected_edge.label_size = 10.0;
                        }
                    });

                    ui.add_space(UI_SPACE);

                    ui.horizontal(|ui| {
                        ui.add(
                            DragValue::new(&mut selected_edge.padding_x)
                                .range(MIN_EDGE_LABEL_PADDING..=MAX_EDGE_LABEL_PADDING)
                                .speed(1.0)
                                .prefix("X: "),
                        );
                        if ui.button("⟲").clicked() {
                            selected_edge.padding_x = 0.0;
                        }
                    });

                    ui.add_space(UI_SPACE);

                    ui.horizontal(|ui| {
                        ui.add(
                            DragValue::new(&mut selected_edge.padding_y)
                                .range(MIN_EDGE_LABEL_PADDING..=MAX_EDGE_LABEL_PADDING)
                                .speed(1.0)
                                .prefix("Y: "),
                        );
                        if ui.button("⟲").clicked() {
                            selected_edge.padding_y = 0.0;
                        }
                    });
                });
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
                    graph_workspace.remove_selected_edge();
                }
            });
        }
    }
}
