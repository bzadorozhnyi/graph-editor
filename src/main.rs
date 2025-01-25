use eframe::egui::{self};
use graph_editor_egui::{
    canvas::Canvas,
    edges_table::EdgesTable,
    graph::{Graph, Node},
    node_editor::NodeEditor,
};

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Graph editor",
        native_options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    graph: Graph,
    canvas: Canvas,
    node_editor: NodeEditor,
    node_editor_open: bool,
    edges_table: EdgesTable,
    edges_table_open: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            graph: Graph::new(),
            canvas: Canvas::new(),
            node_editor: NodeEditor::new(),
            node_editor_open: false,
            edges_table: EdgesTable::new(),
            edges_table_open: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("Node Editor").clicked() {
                    self.node_editor_open = !self.node_editor_open;
                }
                if ui.button("Edges Table").clicked() {
                    self.edges_table_open = !self.edges_table_open;
                }
            });

            if ui.button("New").clicked() {
                self.graph.add_node(Node::new());
            }

            self.canvas.setup(ui);
            self.canvas.handle_draging(&mut self.graph);
            self.canvas.handle_node_selection(&mut self.graph);

            let edge_created = self.canvas.handle_edge_creation(&mut self.graph);
            // if edge_created is true => we clicked on edge's end => ignore this
            if !edge_created {
                self.canvas.handle_setting_edge_start(&self.graph);
            }

            self.node_editor
                .show(ctx, &mut self.node_editor_open, &mut self.graph);

            self.edges_table
                .show(ctx, &mut self.edges_table_open, &mut self.graph);

            self.canvas.draw_possible_edge(&self.graph);
            self.canvas.draw_edges(&self.graph);
            self.canvas.draw_nodes(&self.graph);
        });
    }
}
