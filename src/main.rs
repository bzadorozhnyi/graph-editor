use eframe::egui::{self, color_picker::color_edit_button_rgba, Color32, Sense, Slider, Stroke};
use graph_editor_egui::graph::{Edge, Graph, Node};

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
    dragging: Option<usize>,
    selected_node: Option<usize>,
    new_edge_start: Option<usize>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            graph: Graph::new(),
            dragging: None,
            selected_node: None,
            new_edge_start: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("New").clicked() {
                self.graph.add_node(Node::new());
            }

            // Set up canvas
            let size = ctx.screen_rect().size();
            let (response, painter) = ui.allocate_painter(size, Sense::click_and_drag());
            let rect = response.rect;
            painter.rect_filled(rect, 0.0, Color32::from_rgb(255, 255, 255));

            // handle draging
            if let Some(mouse_pos) = response.interact_pointer_pos() {
                if let Some(index) = self.dragging {
                    self.graph.nodes_mut()[index].position = mouse_pos;
                } else {
                    for (index, node) in self.graph.nodes().iter().enumerate() {
                        if node.position.distance(mouse_pos) < node.radius {
                            self.dragging = Some(index);
                        }
                    }
                }
            } else {
                self.dragging = None;
            }

            // handle node selection
            if let Some(mouse_pos) = response.interact_pointer_pos() {
                for (index, node) in self.graph.nodes().iter().enumerate() {
                    if node.position.distance(mouse_pos) < node.radius {
                        self.selected_node = Some(index);
                    }
                }
            }

            // handle edge creation
            let mut edge_created = false;
            if let (Some(edge_start), Some(mouse_pos)) =
                (self.new_edge_start, response.interact_pointer_pos())
            {
                if response.secondary_clicked() {
                    let mut edge_end = None;
                    for (index, node) in self.graph.nodes().iter().enumerate() {
                        if index != edge_start && node.position.distance(mouse_pos) < node.radius {
                            edge_end = Some(index);
                            break;
                        }
                    }

                    if let Some(edge_end) = edge_end {
                        self.graph.edges_mut().push(Edge::new(edge_start, edge_end));
                        self.new_edge_start = None;
                        edge_created = true;
                    }
                }
            }

            // handle setting edge start
            // if edge_created is true => we clicked on edge's end => ignore this
            if response.secondary_clicked() && !edge_created {
                if let Some(mouse_pos) = response.interact_pointer_pos() {
                    for (index, node) in self.graph.nodes().iter().enumerate() {
                        if node.position.distance(mouse_pos) < node.radius {
                            self.new_edge_start = Some(index);
                            break;
                        }
                    }
                }
            }

            // draw line between edge's start and mouse position
            if let (Some(edge_start), Some(mouse_pos)) = (self.new_edge_start, response.hover_pos())
            {
                painter.line_segment(
                    [self.graph.nodes()[edge_start].position, mouse_pos],
                    Stroke::new(2.0, Color32::BLACK),
                );
            }

            for edge in self.graph.edges() {
                let (start, end) = (
                    &self.graph.nodes()[edge.start_index],
                    &self.graph.nodes()[edge.end_index],
                );

                painter.line_segment(
                    [start.position, end.position],
                    Stroke::new(2.0, Color32::BLACK),
                );
            }

            for node in self.graph.nodes() {
                node.draw(&painter);
            }

            if let Some(selected_node) = self.selected_node {
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
