mod editor_variant;
mod file_operation;

use std::path::PathBuf;

use crate::{
    app::{editor_variant::EditorVariant, file_operation::FileOperation},
    canvas::Canvas,
    comment_line::{editor::CommentsEditor, group::CommentsGroup},
    edge_editor::EdgeEditor,
    edges_table::EdgesTable,
    error::GraphEditorError,
    graph::Graph,
    node_editor::NodeEditor,
    toast::Toast,
    utils::image::{crop_color_image, save_color_image_to_png},
};
use eframe::egui::{self, ColorImage, Context, Margin, SidePanel, Ui, UserData, ViewportCommand};
use egui_file_dialog::FileDialog;

pub struct GraphEditor {
    graph: Graph,
    comment_lines: CommentsGroup,
    canvas: Canvas,
    node_editor: NodeEditor,
    edges_table: EdgesTable,
    edge_editor: EdgeEditor,
    comments_editor: CommentsEditor,
    selected_editor: EditorVariant,
    file_dialog: FileDialog,
    file_operation: FileOperation,
    current_file: Option<PathBuf>,
    toast: Option<Toast>,
    taking_screenshot: bool,
    screenshot: Option<ColorImage>,
}

impl Default for GraphEditor {
    fn default() -> Self {
        Self {
            graph: Graph::new(),
            comment_lines: CommentsGroup::new(),
            canvas: Canvas::new(),
            node_editor: NodeEditor,
            edges_table: EdgesTable,
            edge_editor: EdgeEditor,
            comments_editor: CommentsEditor::new(),
            selected_editor: EditorVariant::Node,
            file_dialog: FileDialog::new(),
            file_operation: FileOperation::None,
            current_file: None,
            toast: None,
            taking_screenshot: false,
            screenshot: None,
        }
    }
}

impl eframe::App for GraphEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_menu(ui);
            self.show_editor_panel(ui);
            self.show_edges_panel(ui);

            if let Err(err) = self.handle_file_operation(ui) {
                self.handle_error(err);
            }

            self.canvas.setup(ctx, ui);

            self.handle_interaction_logic();

            self.canvas
                .draw_components(&self.graph, &self.comment_lines, ui);

            self.show_toast(ui);

            if self.taking_screenshot {
                if let Err(err) = self.take_screenshot(ui) {
                    self.handle_error(err);
                };
            }
        });
    }
}

impl GraphEditor {
    fn show_menu(&mut self, ui: &mut Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                    self.file_operation = FileOperation::FileOpen;
                    self.file_dialog.pick_file();
                }
                if ui.button("Save").clicked() {
                    self.file_operation = FileOperation::FileSave;
                    if self.current_file.is_none() {
                        self.file_dialog.save_file();
                    }
                }
                if ui.button("Save as").clicked() {
                    self.file_operation = FileOperation::FileSaveAs;
                    self.file_dialog.save_file();
                }
            });

            if ui.button("New").clicked() {
                self.graph.add_node();
            }
            ui.selectable_value(&mut self.selected_editor, EditorVariant::Node, "Node");
            ui.selectable_value(&mut self.selected_editor, EditorVariant::Edge, "Edge");
            ui.selectable_value(
                &mut self.selected_editor,
                EditorVariant::CommentLine,
                "Comment line",
            );

            if ui.button("Screenshot").clicked() {
                self.taking_screenshot = true;
            }
        });
    }

    fn show_editor_panel(&mut self, ui: &mut Ui) {
        SidePanel::right("editor_panel")
            .exact_width(250.0)
            .show(ui.ctx(), |ui| {
                egui::Frame::new()
                    .inner_margin(Margin::same(4))
                    .show(ui, |ui| match self.selected_editor {
                        EditorVariant::Node => {
                            self.node_editor.ui(ui, &mut self.graph);
                        }
                        EditorVariant::Edge => {
                            self.edge_editor.ui(ui, &mut self.graph);
                        }
                        EditorVariant::CommentLine => {
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

    fn handle_file_operation(&mut self, ui: &mut Ui) -> Result<(), GraphEditorError> {
        self.file_dialog.update(ui.ctx());

        match self.file_operation {
            FileOperation::FileOpen => {
                if let Some(file_path) = self.file_dialog.take_picked() {
                    self.graph = Graph::try_from(&file_path)?;

                    self.update_current_file(ui, file_path);
                    self.file_operation = FileOperation::None;
                }
            }
            FileOperation::FileSave => {
                let dialog_picked_file = self.file_dialog.take_picked();

                let file_path = if self.current_file.is_some() {
                    self.current_file.clone().unwrap()
                } else if dialog_picked_file.is_some() {
                    dialog_picked_file.unwrap()
                } else {
                    // nothing to save here
                    return Ok(());
                };

                self.graph.save_to_file(&file_path)?;

                self.toast = Some(Toast::success("Saved successfully"));
                self.file_operation = FileOperation::None;
            }
            FileOperation::FileSaveAs => {
                if let Some(file_path) = self.file_dialog.take_picked() {
                    self.graph.save_to_file(&file_path)?;

                    self.update_current_file(ui, file_path);

                    self.toast = Some(Toast::success("Saved successfully"));
                    self.file_operation = FileOperation::None;
                }
            }
            FileOperation::ScreenshotSave => {
                if let Some(file_path) = self.file_dialog.take_picked() {
                    if let Some(image) = &self.screenshot {
                        save_color_image_to_png(file_path, image)
                            .map_err(|_| GraphEditorError::FailedTakeScreenshot)?;
                        self.screenshot = None;
                        self.file_operation = FileOperation::None;
                    }
                }
            }
            FileOperation::None => {}
        }

        Ok(())
    }

    fn handle_interaction_logic(&mut self) {
        if self.selected_editor == EditorVariant::CommentLine {
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

    fn handle_error(&mut self, err: GraphEditorError) {
        self.toast = Some(Toast::error(err.message()))
    }

    fn show_toast(&mut self, ui: &mut Ui) {
        if let Some(toast) = &self.toast {
            if toast.is_expired() {
                self.toast = None;
            } else {
                toast.show(ui);
            }
        }
    }
}

impl GraphEditor {
    fn take_screenshot(&mut self, ui: &mut Ui) -> Result<(), GraphEditorError> {
        ui.ctx()
            .send_viewport_cmd(egui::ViewportCommand::Screenshot(UserData::default()));

        let image = ui.ctx().input(|i| {
            i.events
                .iter()
                .filter_map(|e| {
                    if let egui::Event::Screenshot { image, .. } = e {
                        Some(image.clone())
                    } else {
                        None
                    }
                })
                .last()
        });

        if let Some(image) = image {
            self.taking_screenshot = false;

            let image = crop_color_image(
                &image,
                self.canvas.painter_rect(),
                self.canvas.pixels_per_point(),
            )
            .ok_or(GraphEditorError::FailedTakeScreenshot)?;

            self.file_operation = FileOperation::ScreenshotSave;
            self.file_dialog.save_file();
            self.screenshot = Some(image);
        }

        Ok(())
    }

    fn update_window_title(&self, ctx: &Context, title: &str) {
        ctx.send_viewport_cmd(ViewportCommand::Title(title.to_owned()));
    }

    fn update_current_file(&mut self, ui: &mut Ui, file_path: PathBuf) {
        self.update_window_title(ui.ctx(), &format!("Graph Editor | {}", file_path.display()));
        self.current_file = Some(file_path);
    }
}
