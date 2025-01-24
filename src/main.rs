use eframe::egui::{self, color_picker::color_edit_button_rgba, Slider};
use graph_editor_egui::{
    canvas::Canvas,
    graph::{Graph, Node},
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
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            graph: Graph::new(),
            canvas: Canvas::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("New").clicked() {
                self.graph.add_node(Node::new());
            }

            self.canvas.setup(ui);
            self.canvas.handle_draging(&mut self.graph);
            self.canvas.handle_node_selection(&self.graph);

            let edge_created = self.canvas.handle_edge_creation(&mut self.graph);
            // if edge_created is true => we clicked on edge's end => ignore this
            if !edge_created {
                self.canvas.handle_setting_edge_start(&self.graph);
            }

            self.canvas.draw_possible_edge(&self.graph);
            self.canvas.draw_edges(&self.graph);
            self.canvas.draw_nodes(&self.graph);

            if let Some(selected_node) = self.canvas.selected_node() {
                egui::Window::new("Node").collapsible(true).show(ctx, |ui| {
                    ui.label("Node");
                    ui.separator();
                    ui.text_edit_singleline(&mut self.graph.nodes_mut()[selected_node].label);
                    ui.separator();
                    ui.add(
                        Slider::new(
                            &mut self.graph.nodes_mut()[selected_node].radius,
                            10.0..=100.0,
                        )
                        .text("Size"),
                    );
                    ui.separator();

                    ui.horizontal(|ui| {
                        color_edit_button_rgba(
                            ui,
                            &mut self.graph.nodes_mut()[selected_node].color,
                            egui::color_picker::Alpha::Opaque,
                        );
                        ui.label("Color");
                    });
                });
            }
        });
    }
}
