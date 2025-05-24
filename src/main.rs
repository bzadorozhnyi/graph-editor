use std::{
    fs::File,
    io::{BufReader, Write},
};

use eframe::egui::{self, Margin, SidePanel, Ui};
use egui_file_dialog::FileDialog;
use graph_editor_egui::{
    canvas::Canvas,
    comment_line::{editor::CommentsEditor, group::CommentsGroup},
    edge_editor::EdgeEditor,
    edges_table::EdgesTable,
    graph::Graph,
    node_editor::NodeEditor,
};

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
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

enum FileOperation {
    Open,
    Save,
    None,
}

struct MyApp {
    graph: Graph,
    comment_lines: CommentsGroup,
    canvas: Canvas,
    node_editor: NodeEditor,
    edges_table: EdgesTable,
    edge_editor: EdgeEditor,
    comments_editor: CommentsEditor,
    selected_editor: Editor,
    file_dialog: FileDialog,
    file_operation: FileOperation,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            graph: Graph::new(),
            comment_lines: CommentsGroup::new(),
            canvas: Canvas::new(),
            node_editor: NodeEditor,
            edges_table: EdgesTable,
            edge_editor: EdgeEditor,
            comments_editor: CommentsEditor::new(),
            selected_editor: Editor::Node,
            file_dialog: FileDialog::new(),
            file_operation: FileOperation::None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_menu(ui);
            self.show_editor_panel(ui);
            self.show_edges_panel(ui);

            self.handle_file_operation(ui);

            self.canvas.setup(ctx, ui);

            self.handle_interaction_logic();

            self.canvas
                .draw_components(&self.graph, &self.comment_lines, ui);
        });
    }
}

impl MyApp {
    fn show_menu(&mut self, ui: &mut Ui) {
        egui::Frame::new()
            .inner_margin(Margin::symmetric(8, 0))
            .show(ui, |ui| {
                // TODO: Move menu to separated component
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Save").clicked() {
                            self.file_operation = FileOperation::Save;
                            self.file_dialog.save_file();
                        }
                        if ui.button("Open").clicked() {
                            self.file_operation = FileOperation::Open;
                            self.file_dialog.pick_file();
                        }
                    });

                    if ui.button("New").clicked() {
                        self.graph.add_node();
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
    }

    fn show_editor_panel(&mut self, ui: &mut Ui) {
        SidePanel::right("editor_panel")
            .exact_width(250.0)
            .show(ui.ctx(), |ui| {
                egui::Frame::new()
                    .inner_margin(Margin::same(4))
                    .show(ui, |ui| match self.selected_editor {
                        Editor::Node => {
                            self.node_editor.ui(ui, &mut self.graph);
                        }
                        Editor::Edge => {
                            self.edge_editor.ui(ui, &mut self.graph);
                        }
                        Editor::CommentLine => {
                            self.comments_editor.ui(ui, &mut self.comment_lines);
                        }
                    });
            });
    }

    fn show_edges_panel(&mut self, ui: &mut Ui) {
        egui::TopBottomPanel::bottom("edges_panel")
            .resizable(true)
            .min_height(10.0)
            .show_separator_line(true)
            .show(ui.ctx(), |ui| {
                self.edges_table.ui(ui, &mut self.graph);
            });
    }

    fn handle_file_operation(&mut self, ui: &mut Ui) {
        self.file_dialog.update(ui.ctx());

        if let Some(file_path) = self.file_dialog.take_picked() {
            match self.file_operation {
                FileOperation::Open => {
                    let file = File::open(file_path).unwrap();
                    let reader = BufReader::new(file);

                    // TODO: check if operation successful
                    self.graph = serde_json::from_reader(reader).unwrap();
                }
                FileOperation::Save => {
                    let graph_json = serde_json::to_string_pretty(&self.graph);

                    match graph_json {
                        Ok(value) => {
                            let mut file = File::create(file_path).unwrap();
                            // TODO: check if operation successful
                            file.write_all(value.as_bytes()).unwrap();
                        }
                        Err(_) => {
                            // TODO: replace with pop-up message
                            println!("Not saved")
                        }
                    }
                }
                FileOperation::None => {}
            }
            self.file_operation = FileOperation::None;
        }
    }

    fn handle_interaction_logic(&mut self) {
        if self.selected_editor == Editor::CommentLine {
            if self.comments_editor.draw_mode_active() {
                self.canvas.handle_comment_draw(
                    self.comments_editor.selected_stroke(),
                    &mut self.comment_lines,
                );
            }
            if self.comments_editor.erase_mode_active() {
                self.canvas.handle_comment_erase(&mut self.comment_lines);
            }
        } else {
            self.canvas.handle_node_draging(&mut self.graph);
            self.canvas.handle_node_selection(&mut self.graph);

            let edge_created = self.canvas.handle_edge_creation(&mut self.graph);
            // if edge_created is true => we clicked on edge's end => ignore this
            if !edge_created {
                self.canvas.handle_setting_edge_start(&self.graph);
            }
        }
    }
}
