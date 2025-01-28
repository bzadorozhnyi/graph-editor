use eframe::egui;
use egui_extras::{Column, TableBuilder};

use crate::graph::{edge::EdgeId, Graph};

pub struct EdgesTable {}

impl EdgesTable {
    pub fn new() -> Self {
        Self {}
    }

    pub fn name(&self) -> &'static str {
        "Edges Table"
    }

    pub fn show(&mut self, ctx: &eframe::egui::Context, open: &mut bool, graph: &mut Graph) {
        egui::Window::new(self.name())
            .open(open)
            .collapsible(false)
            .show(ctx, |ui| {
                self.ui(ui, graph);
            });
    }

    fn ui(&mut self, ui: &mut egui::Ui, graph: &mut Graph) {
        let table = TableBuilder::new(ui)
            .cell_layout(egui::Layout::top_down(egui::Align::Center))
            .column(Column::auto())
            .column(Column::remainder().resizable(true))
            .column(Column::remainder().resizable(true))
            .column(Column::remainder().resizable(true))
            .sense(egui::Sense::click());

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("⬆");
                });
                header.col(|ui| {
                    ui.strong("Start");
                });
                header.col(|ui| {
                    ui.strong("End");
                });
                header.col(|ui| {
                    ui.strong("Label");
                });
            })
            .body(|body| {
                // used to get edges in same order (after removing, adding etc.)
                let ids: Vec<_> = graph.edges().keys().cloned().collect();

                // rows is more efficient, than row
                // https://docs.rs/egui_extras/0.30.0/egui_extras/struct.TableBody.html#method.rows
                // that's why using ids - to keep edges order
                body.rows(20.0, graph.edges().len(), |mut row| {
                    let row_index = row.index();
                    let edge_id = &ids[row_index];

                    if let Some(selected_id) = graph.selected_edge_id() {
                        row.set_selected(selected_id == edge_id);
                    }

                    row.col(|ui| {
                        ui.checkbox(
                            &mut graph.edge_mut(edge_id).unwrap().oriented,
                            "",
                        );
                    });
                    row.col(|ui| {
                        ui.label(&graph.nodes()[&graph.edges()[&edge_id].start_id].label);
                    });
                    row.col(|ui| {
                        ui.label(&graph.nodes()[&graph.edges()[&edge_id].end_id].label);
                    });
                    row.col(|ui| {
                        ui.label(&graph.edges().get(edge_id).unwrap().label);
                    });

                    self.toggle_row_selection(edge_id, &row.response(), graph);
                });
            });
    }

    fn toggle_row_selection(
        &mut self,
        edge_id: &EdgeId,
        row_response: &egui::Response,
        graph: &mut Graph,
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
