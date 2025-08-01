use std::collections::HashMap;

use eframe::{
    egui::{
        self, vec2, Color32, FontSelection, Painter, Pos2, Rect, Response, Rgba, RichText, Sense,
        Shape, Stroke, Ui, Vec2, WidgetText,
    },
    emath::Rot2,
    epaint::{CubicBezierShape, QuadraticBezierShape, TextShape},
};

use crate::{
    comment_line::{group::CommentsGroup, CommentLine},
    consts::{ARROW_HALF_ANGLE, ARROW_LEN_COEF, CONTROL_OFFSET, DELTA_ANGLE, MIN_NODE_SIZE},
    graph::{Edge, Graph, Node, NodeId},
};

#[derive(Default)]
pub struct Canvas {
    response: Option<Response>,
    painter: Option<Painter>,
}

// creation, setup and utils
impl Canvas {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn response(&self) -> &Response {
        self.response
            .as_ref()
            .expect("Canvas::setup() must be called first!")
    }

    fn painter(&self) -> &Painter {
        self.painter
            .as_ref()
            .expect("Canvas::setup() must be called first!")
    }

    pub fn painter_rect(&self) -> Rect {
        self.painter().clip_rect()
    }

    pub fn pixels_per_point(&self) -> f32 {
        self.painter().pixels_per_point()
    }

    pub fn setup(&mut self, ctx: &eframe::egui::Context, ui: &mut Ui) {
        let size = ctx.available_rect().shrink2(vec2(8.0, 18.0)).size();

        let (response, painter) = ui.allocate_painter(size, Sense::click_and_drag());
        painter.rect_filled(response.rect, 0.0, Color32::WHITE);

        self.response = Some(response);
        self.painter = Some(painter);
    }

    pub fn set_cursor_icon(&self, cursor_icon: egui::CursorIcon) {
        self.response().ctx.set_cursor_icon(cursor_icon);
    }
}

// nodes
impl Canvas {
    /// Evaluate new position of node, which satisfy painter's bounds constraints
    pub fn bounds_constraint_correction(&self, node: &Node, pointer_pos: Pos2) -> Pos2 {
        let canvas_rect = self.response().rect;

        let new_x = if pointer_pos.x - node.size < canvas_rect.min.x {
            canvas_rect.min.x + node.size
        } else if pointer_pos.x + node.size > canvas_rect.max.x {
            canvas_rect.max.x - node.size
        } else {
            pointer_pos.x
        };

        let new_y = if pointer_pos.y - node.size < canvas_rect.min.y {
            canvas_rect.min.y + node.size
        } else if pointer_pos.y + node.size > canvas_rect.max.y {
            canvas_rect.max.y - node.size
        } else {
            pointer_pos.y
        };

        Pos2::new(new_x, new_y)
    }

    /// Draw all nodes.
    fn draw_nodes(&mut self, graph: &Graph) {
        for node in graph.nodes().values() {
            node.draw(self.painter());
        }
    }
}

// edges
impl Canvas {
    /// Draw possible edge from new_edge_start node to pointer pos.
    fn draw_possible_edge(&mut self, new_edge_start: Option<NodeId>, graph: &Graph) {
        if let (Some(edge_start), Some(pointer_pos)) = (new_edge_start, self.response().hover_pos())
        {
            self.set_cursor_icon(egui::CursorIcon::PointingHand);
            let start_node = &graph.nodes()[&edge_start];

            if start_node.position.distance(pointer_pos) < start_node.size {
                // draw loop
                // Start point is north of node
                let start_pos = start_node.position - Vec2::new(0.0, start_node.size);
                // End point is west of node
                let end_pos = start_node.position - Vec2::new(start_node.size, 0.0);

                // Calc offset based on node size
                let offset = CONTROL_OFFSET * (start_node.size / MIN_NODE_SIZE);

                // Calc controls for curve
                let control1 = start_pos - Vec2::new(0.0, offset);
                let control2 = end_pos - Vec2::new(offset, 0.0);

                self.painter().add(CubicBezierShape::from_points_stroke(
                    [start_pos, control1, control2, end_pos],
                    false,
                    Color32::TRANSPARENT,
                    Stroke::new(2.0, Color32::BLACK),
                ));
            } else {
                self.painter().line_segment(
                    [start_node.position, pointer_pos],
                    Stroke::new(2.0, Color32::BLACK),
                );
            }
        }
    }

    /// Calculate border intersection to draw an edge on the boundary of nodes
    fn calculate_border_intersection(&self, node1: &Node, node2: &Node) -> (Pos2, Pos2) {
        let direction = (node2.position - node1.position).normalized();

        let start = node1.border_point_in_direction(direction);
        let end = node2.border_point_in_direction(-direction);

        (start, end)
    }

    /// Draw edge label.
    fn draw_edge_label(&self, ui: &mut Ui, edge: &Edge, start: Pos2, control: Pos2, end: Pos2) {
        let text = WidgetText::RichText(RichText::new(&edge.label).size(edge.label_size));
        let text_galley = text.into_galley(ui, None, f32::INFINITY, FontSelection::Default);
        let galley_size = text_galley.size();

        let direction = (end - start).normalized();
        let angle = if direction.x <= 0.0 {
            // add PI to autorotate label
            direction.angle() + std::f32::consts::PI
        } else {
            direction.angle()
        };

        // Compute rotated offset
        let half_width = (galley_size.x + edge.padding_x) / 2.0;
        let half_height = (galley_size.y + edge.padding_y) / 2.0;

        // Offset to center the rotated text
        let offset_x = half_width * angle.cos() - half_height * angle.sin();
        let offset_y = half_width * angle.sin() + half_height * angle.cos();

        // Adjust the position to properly center the text
        let centered_position = control - Vec2::new(offset_x, offset_y);

        self.painter().add(
            TextShape::new(centered_position, text_galley.clone(), Color32::BLACK)
                .with_angle(angle),
        );
    }

    /// Draw edge arrow (for oriented edges).
    fn draw_arrow(&self, start: Pos2, end: Pos2, color: Rgba, width: f32) {
        let direction = (end - start).normalized();
        let rotation = Rot2::from_angle(ARROW_HALF_ANGLE);

        let arrow_left = end - ARROW_LEN_COEF * width * (rotation * direction);
        let arrow_right = end - ARROW_LEN_COEF * width * (rotation.inverse() * direction);

        self.painter().add(Shape::convex_polygon(
            vec![end - 0.6 * width * direction, arrow_left, arrow_right],
            color,
            Stroke::new(width, color),
        ));
    }

    /// Draw loop edge
    fn draw_loop(&self, ui: &mut Ui, graph: &Graph, edge: &Edge, shift: f32) {
        let node = &graph.nodes()[&edge.start_id];

        let rotation_angle = edge.loop_rotation_angle.to_radians();

        // Calculate border points based on rotation angle.
        // Start point is north of node + rotation angle.
        let start =
            node.rotate_border_point(node.position - Vec2::new(0.0, node.size), rotation_angle);
        // End point is west of node + rotation angle.
        let end =
            node.rotate_border_point(node.position - Vec2::new(node.size, 0.0), rotation_angle);

        // Calc direction of vectors:
        // direction1: start -> node.center (node.position)
        let direction1 = (node.position - start).normalized();
        // direction2: end   -> node.center (node.position)
        let direction2 = (node.position - end).normalized();

        // Calc offset based on node size and shift (possible multiple loops)
        let offset = CONTROL_OFFSET * (node.size / MIN_NODE_SIZE) * (1.0 + shift);
        // Calc controls for curve
        let control1 = start - direction1 * offset;
        let control2 = end - direction2 * offset;

        let curve = CubicBezierShape::from_points_stroke(
            [start, control1, control2, end],
            false,
            Color32::TRANSPARENT,
            Stroke::new(edge.width, edge.color),
        );

        // Calc curve middle to place label in center of edge
        let curve_middle = curve.sample(0.5);
        self.painter().add(curve);

        if !edge.label.is_empty() {
            self.draw_edge_label(ui, edge, start, curve_middle, end);
        }
    }

    /// Draw edge.
    fn draw_edge(&self, ui: &mut Ui, graph: &Graph, edge: &Edge, shift: f32) {
        let direction_sign = if edge.start_id < edge.end_id {
            -1.0
        } else {
            1.0
        };

        let (node_start, node_end) = (&graph.nodes()[&edge.start_id], &graph.nodes()[&edge.end_id]);
        let (start, end) = self.calculate_border_intersection(node_start, node_end);

        // Calc edge start and end to avoid edges overlaping
        // based on shift and direction_sign
        let alpha = DELTA_ANGLE * shift * direction_sign;
        let start = node_start.rotate_border_point(start, alpha);
        let end = node_end.rotate_border_point(end, -alpha);

        // Calc edge control for curve
        let direction = direction_sign * (start - end).normalized();
        let midpoint = Pos2::new((start.x + end.x) / 2.0, (start.y + end.y) / 2.0);
        let control = midpoint + direction.rot90() * shift * CONTROL_OFFSET;

        let curve = QuadraticBezierShape::from_points_stroke(
            [start, control, end],
            false,
            Color32::TRANSPARENT,
            Stroke::new(edge.width, edge.color),
        );

        // Calc curve middle to place label in center of edge
        let curve_control = curve.sample(0.5);
        self.painter().add(curve);

        if !edge.label.is_empty() {
            self.draw_edge_label(ui, edge, start, curve_control, end);
        }

        if edge.oriented {
            self.draw_arrow(control, end, edge.color, edge.width);
        }
    }

    /// Draw all edges.
    fn draw_edges(&mut self, ui: &mut Ui, graph: &Graph) {
        let mut grouped_edges = HashMap::<(NodeId, NodeId), Vec<&Edge>>::new();

        for edge in graph.edges().values() {
            let edge_order = if edge.start_id < edge.end_id {
                (edge.start_id, edge.end_id)
            } else {
                (edge.end_id, edge.start_id)
            };

            grouped_edges
                .entry(edge_order)
                .and_modify(|v| v.push(edge))
                .or_insert(vec![edge]);
        }

        for ((start_id, end_id), edges) in grouped_edges {
            if start_id == end_id {
                // iterate over loops
                for (index, &edge) in edges.iter().enumerate() {
                    self.draw_loop(ui, graph, edge, index as f32);
                }
            } else {
                // Calc shifts to avoid edges overlapping
                let edges_number = (edges.len() / 2) as isize;
                let shifting =
                    (-edges_number..=edges_number).filter(|&n| edges.len() % 2 != 0 || n != 0);

                for (&edge, shift) in edges.iter().zip(shifting) {
                    self.draw_edge(ui, graph, edge, shift as f32);
                }
            }
        }
    }
}

// comment lines
impl Canvas {
    pub fn handle_comment_draw(&mut self, stroke: Stroke, comment_lines: &mut CommentsGroup) {
        self.set_cursor_icon(egui::CursorIcon::Cell);

        if comment_lines.is_empty() {
            comment_lines.insert(CommentLine::from(stroke));
        }

        let pointer_pos = self.response().interact_pointer_pos();

        let current_line = comment_lines.last_added_mut().unwrap();

        // update selected params
        if current_line.is_empty() {
            current_line.stroke = stroke;
        }

        if let Some(pointer_pos) = pointer_pos {
            // if last point of line is not in pointer_pos
            // => line is extended (added segment)
            if current_line.points.last() != Some(&pointer_pos) {
                current_line.points.push(pointer_pos);
            }
        } else if !current_line.is_empty() {
            comment_lines.insert(CommentLine::new());
        }
    }

    pub fn handle_comment_erase(&mut self, comment_lines: &mut CommentsGroup) {
        let square_center = match self.response().hover_pos() {
            Some(center) => center,
            None => return,
        };

        self.set_cursor_icon(egui::CursorIcon::None);

        // Create eraser square in hover_pos
        let hover_square = Rect::from_center_size(square_center, Vec2::new(10.0, 10.0));

        self.painter().rect_stroke(
            hover_square,
            0.0,
            Stroke::new(1.0, Color32::BLACK),
            egui::StrokeKind::Outside,
        );

        if let Some(pointer_pos) = self.response().interact_pointer_pos() {
            // Create eraser square in pointer_pos
            let interact_square = Rect::from_center_size(pointer_pos, Vec2::new(10.0, 10.0));

            self.painter().rect_stroke(
                interact_square,
                0.0,
                Stroke::new(1.0, Color32::BLACK),
                egui::StrokeKind::Outside,
            );

            // Find comment_line intersected by interact_square
            let mut selected_line_id = None;
            for (id, line) in comment_lines.iter() {
                if line.is_intersect_square(interact_square) {
                    selected_line_id = Some(id);
                    break;
                }
            }

            // Erase comment_line intersected by interact_square
            if let Some(id) = selected_line_id {
                comment_lines.remove(*id);
            }
        }
    }

    /// Draw all comment lines.
    fn draw_comment_lines(&self, comment_lines: &CommentsGroup) {
        let lines = comment_lines
            .iter()
            .filter(|(_, line)| line.len() >= 2)
            .map(|(_, line)| egui::Shape::line(line.points.clone(), line.stroke));

        self.painter().extend(lines);
    }

    /// Draw possible edge, all nodes and edges, comment lines.
    pub fn draw_components(
        &mut self,
        graph: &Graph,
        new_edge_start: Option<NodeId>,
        comment_lines: &CommentsGroup,
        ui: &mut Ui,
    ) {
        self.draw_possible_edge(new_edge_start, graph);
        self.draw_edges(ui, graph);
        self.draw_nodes(graph);
        self.draw_comment_lines(comment_lines);
    }
}
