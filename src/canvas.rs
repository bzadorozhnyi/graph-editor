use eframe::{
    egui::{Color32, Painter, Pos2, Rect, Response, Sense, Stroke, Ui},
    emath::Rot2,
};

use crate::graph::{Edge, Graph, Node, NodeId};

pub struct Canvas {
    response: Option<Response>,
    painter: Option<Painter>,
    painter_area: Rect,
    new_edge_start: Option<NodeId>,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            response: None,
            painter: None,
            painter_area: Rect::ZERO,
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

    pub fn setup(&mut self, ui: &mut Ui, menu_size: f32) {
        let rect = ui.min_rect();
        let rect = rect.with_min_y(rect.min.y + menu_size);

        self.painter_area = rect;
        let size = self.painter_area.size();
        let (response, painter) = ui.allocate_painter(size, Sense::click_and_drag());
        painter.rect_filled(self.painter_area, 0.0, Color32::WHITE);

        self.response = Some(response);
        self.painter = Some(painter);
    }

    /// Evaluate new position of node, which satisfy painter's bounds constraints 
    fn bounds_constraint_correction(&self, node: &Node, mouse_pos: Pos2) -> Pos2 {
        let new_x = if mouse_pos.x - node.radius < self.painter_area.min.x {
            self.painter_area.min.x + node.radius
        } else if mouse_pos.x + node.radius > self.painter_area.max.x {
            self.painter_area.max.x - node.radius
        } else {
            mouse_pos.x
        };

        let new_y = if mouse_pos.y - node.radius < self.painter_area.min.y {
            self.painter_area.min.y + node.radius
        } else if mouse_pos.y + node.radius > self.painter_area.max.y {
            self.painter_area.max.y - node.radius
        } else {
            mouse_pos.y
        };

        Pos2::new(new_x, new_y)
    }

    pub fn handle_draging(&mut self, graph: &mut Graph) {
        if let Some(mouse_pos) = self.response().interact_pointer_pos() {
            if let Some(id) = graph.dragging() {
                let node = graph.nodes().get(&id).unwrap();
                let new_pos = self.bounds_constraint_correction(node, mouse_pos);

                if self.response().rect.contains(new_pos) {
                    graph.nodes_mut().get_mut(&id).unwrap().position = new_pos;
                }
            } else {
                let mut dragging = None;
                for (id, node) in graph.nodes().iter() {
                    if node.position.distance(mouse_pos) < node.radius {
                        dragging = Some(*id);
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
            let mut selected_node_id = None;
            for (id, node) in graph.nodes() {
                if node.position.distance(mouse_pos) < node.radius {
                    selected_node_id = Some(*id);
                    break;
                }
            }
            if selected_node_id.is_some() {
                graph.set_selected_node_id(selected_node_id);
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
                for (id, node) in graph.nodes() {
                    if *id != edge_start && node.position.distance(mouse_pos) < node.radius {
                        edge_end = Some(*id);
                        break;
                    }
                }

                if let Some(edge_end) = edge_end {
                    graph.add_edge(Edge::new(edge_start, edge_end));
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
                for (id, node) in graph.nodes() {
                    if node.position.distance(mouse_pos) < node.radius {
                        self.new_edge_start = Some(*id);
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
            &graph.nodes()[&edge.start_id],
            &graph.nodes()[&edge.end_id],
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
        for edge in graph.edges().values() {
            self.draw_edge(graph, edge);
        }
    }

    pub fn draw_nodes(&mut self, graph: &Graph) {
        for node in graph.nodes().values() {
            node.draw(self.painter());
        }
    }
}
