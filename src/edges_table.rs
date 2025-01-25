use eframe::egui;
use egui_extras::{Column, TableBuilder};

use crate::graph::Graph;

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
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::remainder().resizable(true))
            .column(Column::remainder().resizable(true));

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
            })
            .body(|body| {
                body.rows(20.0, graph.edges().len(), |mut row| {
                    let row_index = row.index();
                    row.col(|ui| {
                        ui.checkbox(&mut graph.edges_mut()[row_index].oriented, "");
                    });
                    row.col(|ui| {
                        ui.label(&graph.nodes()[&graph.edges()[row_index].start_index].label);
                    });
                    row.col(|ui| {
                        ui.label(&graph.nodes()[&graph.edges()[row_index].end_index].label);
                    });
                });
            });
    }
}
