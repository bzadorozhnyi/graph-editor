use eframe::{
    egui::{Color32, Painter, Pos2, Response, Sense, Stroke, Ui},
    emath::Rot2,
};

use crate::graph::{Edge, Graph, Node, NodeId};

pub struct Canvas {
    response: Option<Response>,
    painter: Option<Painter>,
    new_edge_start: Option<NodeId>,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            response: None,
            painter: None,
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
            if let Some(index) = graph.dragging() {
                graph.nodes_mut().get_mut(&index).unwrap().position = mouse_pos;
            } else {
                let mut dragging = None;
                for (index, node) in graph.nodes().iter() {
                    if node.position.distance(mouse_pos) < node.radius {
                        dragging = Some(*index);
                    }
                }

                if dragging.is_some() {
                    graph.set_dragging(dragging);
                }
            }
        } else {
            graph.set_dragging(None);
        }
    }

    pub fn handle_node_selection(&mut self, graph: &mut Graph) {
        if let Some(mouse_pos) = self.response().interact_pointer_pos() {
            let mut selected_node_index = None;
            for (index, node) in graph.nodes() {
                if node.position.distance(mouse_pos) < node.radius {
                    selected_node_index = Some(*index);
                    break;
                }
            }
            if selected_node_index.is_some() {
                graph.set_selected_node_index(selected_node_index);
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
                for (index, node) in graph.nodes() {
                    if *index != edge_start && node.position.distance(mouse_pos) < node.radius {
                        edge_end = Some(*index);
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
                for (index, node) in graph.nodes() {
                    if node.position.distance(mouse_pos) < node.radius {
                        self.new_edge_start = Some(*index);
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
                [graph.nodes()[&edge_start].position, mouse_pos],
                Stroke::new(2.0, Color32::BLACK),
            );
        }
    }

    fn calculate_border_intersection(node1: &Node, node2: &Node) -> (Pos2, Pos2) {
        let direction = (node2.position - node1.position).normalized();

        let start = node1.position + direction * node1.radius;
        let end = node2.position - direction * node2.radius;

        (start, end)
    }

    fn draw_edge(&self, graph: &Graph, edge: &Edge) {
        let (start, end) = Self::calculate_border_intersection(
            &graph.nodes()[&edge.start_index],
            &graph.nodes()[&edge.end_index],
        );

        self.painter()
            .line_segment([start, end], Stroke::new(2.0, Color32::BLACK));

        if edge.oriented {
            let rotation = Rot2::from_angle(std::f32::consts::TAU / 10.0);
            let direction = (end - start).normalized();

            self.painter().line_segment(
                [end, end - 10.0 * (rotation * direction)],
                Stroke::new(2.0, Color32::BLACK),
            );
            self.painter().line_segment(
                [end, end - 10.0 * (rotation.inverse() * direction)],
                Stroke::new(2.0, Color32::BLACK),
            );
        }
    }

    pub fn draw_edges(&mut self, graph: &Graph) {
        for edge in graph.edges() {
            self.draw_edge(graph, edge);
        }
    }

    pub fn draw_nodes(&mut self, graph: &Graph) {
        for node in graph.nodes().values() {
            node.draw(self.painter());
        }
    }
}
