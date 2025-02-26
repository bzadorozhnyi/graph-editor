use eframe::egui::{self, Margin, SidePanel};
use graph_editor_egui::{
    canvas::Canvas,
    comment_line::editor::CommentsEditor,
    edge_editor::EdgeEditor,
    edges_table::EdgesTable,
    graph::{Graph, Node},
    node_editor::NodeEditor,
};

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 675.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Graph editor",
        native_options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(PartialEq)]
enum Editor {
    Node,
    Edge,
    CommentLine,
}

struct MyApp {
    graph: Graph,
    canvas: Canvas,
    node_editor: NodeEditor,
    edges_table: EdgesTable,
    edge_editor: EdgeEditor,
    comments_editor: CommentsEditor,
    selected_editor: Editor,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            graph: Graph::new(),
            canvas: Canvas::new(),
            node_editor: NodeEditor::new(),
            edges_table: EdgesTable::new(),
            edge_editor: EdgeEditor::new(),
            comments_editor: CommentsEditor::new(),
            selected_editor: Editor::Node,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let right_panel_width = 250.0;
            egui::Frame::none()
                .inner_margin(Margin::symmetric(8.0, 0.0))
                .show(ui, |ui| {
                    egui::menu::bar(ui, |ui| {
                        if ui.button("New").clicked() {
                            self.graph.add_node(Node::new());
                        }
                        ui.selectable_value(&mut self.selected_editor, Editor::Node, "Node");
                        ui.selectable_value(&mut self.selected_editor, Editor::Edge, "Edge");
                        ui.selectable_value(
                            &mut self.selected_editor,
                            Editor::CommentLine,
                            "Comment line",
                        );
                    });
                });

            SidePanel::right("menu_panel")
                .exact_width(right_panel_width)
                .show(ctx, |ui| {
                    egui::Frame::none()
                        .inner_margin(Margin::same(4.0))
                        .show(ui, |ui| match self.selected_editor {
                            Editor::Node => {
                                self.node_editor.ui(ui, &mut self.graph);
                            }
                            Editor::Edge => {
                                self.edge_editor.ui(ui, &mut self.graph);
                            }
                            Editor::CommentLine => {
                                self.comments_editor.ui(ui);
                            }
                        });
                });

            egui::TopBottomPanel::bottom("bottom_panel")
                .resizable(true)
                .min_height(10.0)
                .show_separator_line(true)
                .show(ctx, |ui| {
                    self.edges_table.ui(ui, &mut self.graph);
                });

            self.canvas.setup(ctx, ui);

            if self.selected_editor == Editor::CommentLine {
                if self.comments_editor.draw_mode_active() {
                    self.canvas
                        .handle_comment_draw(self.comments_editor.selected_stroke());
                }
                if self.comments_editor.erase_mode_active() {
                    self.canvas.handle_comment_erase();
                }
            } else {
                self.canvas.handle_draging(&mut self.graph);
                self.canvas.handle_node_selection(&mut self.graph);

                let edge_created = self.canvas.handle_edge_creation(&mut self.graph);
                // if edge_created is true => we clicked on edge's end => ignore this
                if !edge_created {
                    self.canvas.handle_setting_edge_start(&self.graph);
                }
            }

            self.canvas.draw_possible_edge(&self.graph);
            self.canvas.draw_edges(ui, &self.graph);
            self.canvas.draw_nodes(&self.graph);
            self.canvas.draw_comment_lines();
        });
    }
}
