use eframe::egui::{self};
use graph_editor_egui::{
    canvas::Canvas,
    edge_editor::EdgeEditor,
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
    edge_editor: EdgeEditor,
    edge_editor_open: bool,
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
            edge_editor: EdgeEditor::new(),
            edge_editor_open: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let bar_resonse = egui::menu::bar(ui, |ui| {
                if ui.button("New").clicked() {
                    self.graph.add_node(Node::new());
                }
                ui.toggle_value(&mut self.node_editor_open, "Node Editor");
                ui.toggle_value(&mut self.edges_table_open, "Edges Table");
                ui.toggle_value(&mut self.edge_editor_open, "Edge Editor");
            });

            self.canvas
                .setup(ui, bar_resonse.response.rect.height() * 1.25);
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

            self.edge_editor
                .show(ctx, &mut self.edge_editor_open, &mut self.graph);

            self.canvas.draw_possible_edge(&self.graph);
            self.canvas.draw_edges(ui, &self.graph);
            self.canvas.draw_nodes(&self.graph);
        });
    }
}
