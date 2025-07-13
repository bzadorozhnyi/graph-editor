use eframe::egui::{self, frame, Margin};
use egui_extras::{Column, TableBuilder};

use crate::{graph::edge::EdgeId, graph_workspace::GraphWorkspace};

pub struct EdgesTable;

impl EdgesTable {
    pub fn name(&self) -> &'static str {
        "Edges Table"
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, graph_workspace: &mut GraphWorkspace) {
        let width = (ui.available_width() - 30.0).max(0.0) / 3.0;

        let table = TableBuilder::new(ui)
            .min_scrolled_height(100.0)
            .cell_layout(egui::Layout::top_down(egui::Align::Center))
            .column(Column::auto().at_least(width).at_most(width).clip(true))
            .column(Column::auto().at_least(width).at_most(width).clip(true))
            .column(Column::auto())
            .column(Column::auto().at_least(width).at_most(width).clip(true))
            .sense(egui::Sense::click());

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Start");
                });
                header.col(|ui| {
                    ui.strong("End");
                });
                header.col(|ui| {
                    ui.strong("⬆");
                });
                header.col(|ui| {
                    ui.strong("Label");
                });
            })
            .body(|body| {
                // used to get edges in same order (after removing, adding etc.)
                let ids: Vec<_> = graph_workspace.edges_ids();

                // rows is more efficient, than row
                // https://docs.rs/egui_extras/0.30.0/egui_extras/struct.TableBody.html#method.rows
                // that's why using ids - to keep edges order
                body.rows(20.0, ids.len(), |mut row| {
                    let row_index = row.index();
                    let edge_id = &ids[row_index];

                    if let Some(selected_id) = graph_workspace.selected_edge_id() {
                        row.set_selected(selected_id == edge_id);
                    }

                    let edge_nodes = graph_workspace.edge_nodes(edge_id).unwrap();

                    row.col(|ui| {
                        ui.label(&edge_nodes.0.label);
                    });
                    row.col(|ui| {
                        ui.label(&edge_nodes.1.label);
                    });

                    let edge = graph_workspace.edge_mut(edge_id).unwrap();
                    row.col(|ui| {
                        if !edge.is_loop() {
                            ui.checkbox(&mut edge.oriented, "");
                        }
                    });
                    row.col(|ui| {
                        frame::Frame::default()
                            .inner_margin(Margin::symmetric(2, 0))
                            .show(ui, |ui| {
                                ui.text_edit_singleline(&mut edge.label);
                            });
                    });

                    self.toggle_row_selection(edge_id, &row.response(), graph_workspace);
                });
            });
    }

    fn toggle_row_selection(
        &mut self,
        edge_id: &EdgeId,
        row_response: &egui::Response,
        graph: &mut GraphWorkspace,
    ) {
        if row_response.clicked() {
            if let Some(selected_id) = graph.selected_edge_id() {
                if selected_id == edge_id {
                    graph.set_selected_edge_id(None);
                } else {
                    graph.set_selected_edge_id(Some(*edge_id));
                }
            } else {
                graph.set_selected_edge_id(Some(*edge_id));
            }
        }
    }
}
