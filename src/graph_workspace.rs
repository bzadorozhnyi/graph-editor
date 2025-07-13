use std::path::PathBuf;

use eframe::egui::{self, Context, Rect, Stroke, Ui};

use crate::{
    canvas::Canvas,
    comment_line::group::CommentsGroup,
    error::GraphEditorError,
    graph::{edge::EdgeId, Edge, Graph, Node, NodeId},
};

#[derive(Default)]
struct InteractionState {
    selected_node_id: Option<NodeId>,
    dragging_node_id: Option<NodeId>,
    selected_edge_id: Option<EdgeId>,
    new_edge_start: Option<NodeId>,
}

#[derive(Default)]
pub struct GraphWorkspace {
    canvas: Canvas,
    comment_lines: CommentsGroup,
    graph: Graph,
    interactions: InteractionState,
}

impl GraphWorkspace {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setup(&mut self, ctx: &Context, ui: &mut Ui) {
        self.canvas.setup(ctx, ui);
    }

    pub fn selected_node_mut(&mut self) -> Option<&mut Node> {
        self.interactions
            .selected_node_id
            .map(|id| self.graph.node_mut(&id).unwrap())
    }

    pub fn set_selected_node_id(&mut self, node_id: Option<NodeId>) {
        self.interactions.selected_node_id = node_id;
    }

    pub fn remove_selected_node(&mut self) {
        if let Some(selected_id) = self.interactions.selected_node_id {
            self.remove_node(selected_id);
        }
    }

    /// Mark one node selected if pointer position same as this node position.
    pub fn handle_node_selection(&mut self) {
        if let Some(pointer_pos) = self.canvas.response().interact_pointer_pos() {
            for (id, node) in self.graph.nodes() {
                if node.is_clicked(pointer_pos) {
                    self.set_selected_node_id(Some(*id));
                    break;
                }
            }
        }
    }

    pub fn dragging_node(&self) -> Option<NodeId> {
        self.interactions.dragging_node_id
    }

    pub fn set_dragging_node(&mut self, dragging: Option<NodeId>) {
        self.interactions.dragging_node_id = dragging;
    }

    /// Handle node dragging.
    pub fn handle_node_draging(&mut self) {
        let pointer_pos = match self.canvas.response().interact_pointer_pos() {
            Some(pos) => pos,
            None => {
                // any node is not node dragging
                self.set_dragging_node(None);
                return;
            }
        };

        self.canvas.set_cursor_icon(egui::CursorIcon::Grabbing);

        // drag selected node to poiter pos
        if let Some(id) = self.dragging_node() {
            let node = self.graph.nodes().get(&id).unwrap();
            self.graph.node_mut(&id).unwrap().position =
                self.canvas.bounds_constraint_correction(node, pointer_pos);

            return;
        }

        // if pointer pos is the same as some node => mark node as dragging node
        for (id, node) in self.graph.nodes().iter() {
            if node.is_clicked(pointer_pos) {
                self.set_dragging_node(Some(*id));
                break;
            }
        }
    }

    pub fn handle_graph_interactions(&mut self) {
        self.handle_node_draging();
        self.handle_node_selection();

        let edge_created = self.handle_edge_creation();
        // if edge_created is true => we clicked on edge's end => ignore this
        if !edge_created {
            self.handle_setting_edge_start();
        }
    }

    /// Handle edge creation.
    /// Return true if edge was created
    pub fn handle_edge_creation(&mut self) -> bool {
        // if Escape pressed => we dont' creating edge anymore
        if self
            .canvas
            .response()
            .ctx
            .input(|i| i.key_pressed(egui::Key::Escape))
        {
            self.interactions.new_edge_start = None;
            return false;
        }

        // if right mouse button was not clicked => ignore
        if !self.canvas.response().secondary_clicked() {
            return false;
        }

        if let Some(edge_start) = self.interactions.new_edge_start {
            let pointer_pos = self.canvas.response().interact_pointer_pos().unwrap();

            // if some node has same pos as pointer
            // then creating edge (edge_start; node)
            for (id, node) in self.graph.nodes() {
                if node.is_clicked(pointer_pos) {
                    self.add_edge(edge_start, *id);
                    self.interactions.new_edge_start = None;

                    return true;
                }
            }
        }

        false
    }

    /// Handle setting start of edge (first selected node).
    /// (Edge is not created at this moment)
    pub fn handle_setting_edge_start(&mut self) {
        if self.canvas.response().secondary_clicked() {
            let pointer_pos = self.canvas.response().interact_pointer_pos().unwrap();

            // if some node has same pos as pointer
            // then set edge start as node id
            for (id, node) in self.graph.nodes() {
                if node.is_clicked(pointer_pos) {
                    self.interactions.new_edge_start = Some(*id);
                    break;
                }
            }
        }
    }

    pub fn run(&mut self, ui: &mut Ui) {
        self.canvas.draw_components(
            &self.graph,
            self.interactions.new_edge_start,
            &self.comment_lines,
            ui,
        );
    }

    pub fn add_node(&mut self) {
        self.graph.add_node();
    }

    pub fn remove_node(&mut self, id: NodeId) {
        if let Some(selected_id) = self.interactions.selected_node_id {
            if id == selected_id {
                self.interactions.selected_node_id = None;
            }
        }

        if let Some(dragging_id) = self.interactions.dragging_node_id {
            if id == dragging_id {
                self.interactions.dragging_node_id = None;
            }
        }

        self.graph.remove_node(id);

        // TODO: check if need this, looks weird
        // meaning we removed edge when deleting edges connected to node
        // but must set selected id = None
        if self.selected_edge().is_none() {
            self.set_selected_edge_id(None);
        }
    }

    pub fn selected_edge_mut(&mut self) -> Option<&mut Edge> {
        self.interactions
            .selected_edge_id
            .map(|id| self.graph.edge_mut(&id).unwrap())
    }

    pub fn selected_edge(&self) -> Option<&Edge> {
        match self.interactions.selected_edge_id {
            Some(id) => self.graph.edges().get(&id),
            None => None,
        }
    }

    pub fn selected_edge_id(&self) -> &Option<EdgeId> {
        &self.interactions.selected_edge_id
    }

    pub fn set_selected_edge_id(&mut self, edge_id: Option<EdgeId>) {
        self.interactions.selected_edge_id = edge_id;
    }

    pub fn remove_selected_edge(&mut self) {
        if let Some(selected_id) = self.interactions.selected_edge_id {
            self.remove_edge(selected_id);
        }
    }

    pub fn remove_edge(&mut self, id: EdgeId) {
        if let Some(selected_id) = self.interactions.selected_edge_id {
            if id == selected_id {
                self.interactions.selected_edge_id = None;
            }
        }

        self.graph.remove_edge(id);
    }

    pub fn node(&self, id: &NodeId) -> Option<&Node> {
        self.graph.node(id)
    }

    pub fn edges_ids(&self) -> Vec<EdgeId> {
        self.graph.edges().keys().cloned().collect()
    }

    pub fn edge(&self, id: &EdgeId) -> Option<&Edge> {
        self.graph.edge(id)
    }

    pub fn edge_mut(&mut self, id: &EdgeId) -> Option<&mut Edge> {
        self.graph.edge_mut(id)
    }

    pub fn edge_nodes(&self, id: &EdgeId) -> Option<(&Node, &Node)> {
        if let Some(edge) = self.edge(id) {
            let start_node = self.node(&edge.start_id).unwrap();
            let end_node = self.node(&edge.end_id).unwrap();

            Some((start_node, end_node))
        } else {
            None
        }
    }

    pub fn add_edge(&mut self, start_id: NodeId, end_id: NodeId) {
        self.graph.add_edge(start_id, end_id);
    }

    pub fn graph_from_file(&mut self, file_path: &PathBuf) -> Result<(), GraphEditorError> {
        self.graph = Graph::try_from(file_path)?;
        Ok(())
    }

    pub fn save_graph_to_file(&mut self, file_path: &PathBuf) -> Result<(), GraphEditorError> {
        self.graph.save_to_file(file_path)
    }

    pub fn canvas_rect(&self) -> Rect {
        self.canvas.painter_rect()
    }

    pub fn canvas_pixels_per_point(&self) -> f32 {
        self.canvas.pixels_per_point()
    }

    pub fn handle_comment_draw(&mut self, stroke: Stroke) {
        self.canvas.handle_comment_draw(
            stroke,
            &mut self.comment_lines,
        );
    }

    pub fn handle_comment_erase(&mut self) {
        self.canvas.handle_comment_erase(&mut self.comment_lines);
    }

    pub fn comment_lines(&mut self) -> &mut CommentsGroup {
        &mut self.comment_lines
    }
}
