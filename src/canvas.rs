use eframe::egui::{Color32, Painter, Response, Sense, Stroke, Ui};

use crate::graph::{Edge, Graph};

pub struct Canvas {
    response: Option<Response>,
    painter: Option<Painter>,
    dragging: Option<usize>,
    selected_node: Option<usize>,
    new_edge_start: Option<usize>,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            response: None,
            painter: None,
            dragging: None,
            selected_node: None,
            new_edge_start: None,
        }
    }

    fn response(&self) -> &Response {
        self.response
            .as_ref()
            .expect("Canvas::setup() must be called first!")
    }

    fn painter(&self) -> &Painter {
        self.painter
            .as_ref()
            .expect("Canvas::setup() must be called first!")
    }

    pub fn setup(&mut self, ui: &mut Ui) {
        let size = ui.ctx().screen_rect().size();
        let (response, painter) = ui.allocate_painter(size, Sense::click_and_drag());
        let rect = response.rect;
        painter.rect_filled(rect, 0.0, Color32::WHITE);

        self.response = Some(response);
        self.painter = Some(painter);
    }

    pub fn handle_draging(&mut self, graph: &mut Graph) {
        if let Some(mouse_pos) = self.response().interact_pointer_pos() {
            if let Some(index) = self.dragging {
                graph.nodes_mut()[index].position = mouse_pos;
            } else {
                for (index, node) in graph.nodes().iter().enumerate() {
                    if node.position.distance(mouse_pos) < node.radius {
                        self.dragging = Some(index);
                    }
                }
            }
        } else {
            self.dragging = None;
        }
    }

    pub fn handle_node_selection(&mut self, graph: &Graph) {
        if let Some(mouse_pos) = self.response().interact_pointer_pos() {
            for (index, node) in graph.nodes().iter().enumerate() {
                if node.position.distance(mouse_pos) < node.radius {
                    self.selected_node = Some(index);
                }
            }
        }
    }

    /// Return true if edge was created
    pub fn handle_edge_creation(&mut self, graph: &mut Graph) -> bool {
        let mut edge_created = false;

        if let (Some(edge_start), Some(mouse_pos)) =
            (self.new_edge_start, self.response().interact_pointer_pos())
        {
            if self.response().secondary_clicked() {
                let mut edge_end = None;
                for (index, node) in graph.nodes().iter().enumerate() {
                    if index != edge_start && node.position.distance(mouse_pos) < node.radius {
                        edge_end = Some(index);
                        break;
                    }
                }

                if let Some(edge_end) = edge_end {
                    graph.edges_mut().push(Edge::new(edge_start, edge_end));
                    self.new_edge_start = None;
                    edge_created = true;
                }
            }
        }

        edge_created
    }

    pub fn handle_setting_edge_start(&mut self, graph: &Graph) {
        if self.response().secondary_clicked() {
            if let Some(mouse_pos) = self.response().interact_pointer_pos() {
                for (index, node) in graph.nodes().iter().enumerate() {
                    if node.position.distance(mouse_pos) < node.radius {
                        self.new_edge_start = Some(index);
                        break;
                    }
                }
            }
        }
    }

    pub fn draw_possible_edge(&mut self, graph: &Graph) {
        if let (Some(edge_start), Some(mouse_pos)) =
            (self.new_edge_start, self.response().hover_pos())
        {
            self.painter().line_segment(
                [graph.nodes()[edge_start].position, mouse_pos],
                Stroke::new(2.0, Color32::BLACK),
            );
        }
    }

    pub fn draw_edges(&mut self, graph: &Graph) {
        for edge in graph.edges() {
            let (start, end) = (
                &graph.nodes()[edge.start_index],
                &graph.nodes()[edge.end_index],
            );

            self.painter().line_segment(
                [start.position, end.position],
                Stroke::new(2.0, Color32::BLACK),
            );
        }
    }

    pub fn draw_nodes(&mut self, graph: &Graph) {
        for node in graph.nodes() {
            node.draw(self.painter());
        }
    }

    pub fn selected_node(&self) -> Option<usize> {
        self.selected_node
    }
}
