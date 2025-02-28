use std::collections::HashMap;

use eframe::{
    egui::{
        self, Align2, Color32, FontFamily, FontId, FontSelection, Painter, Pos2, Rect, Response,
        Rgba, RichText, Sense, Shape, Stroke, Ui, Vec2, WidgetText,
    },
    emath::Rot2,
    epaint::{CubicBezierShape, QuadraticBezierShape, TextShape},
};

use crate::{
    comment_line::{group::CommentsGroup, CommentLine},
    consts::{ARROW_HALF_ANGLE, ARROW_LEN_COEF, CONTROL_OFFSET, DELTA_ANGLE},
    graph::{Edge, Graph, Node, NodeId},
};

pub struct Canvas {
    response: Option<Response>,
    painter: Option<Painter>,
    painter_area: Rect,
    new_edge_start: Option<NodeId>,
    comment_lines: CommentsGroup,
}

// creation, setup and utils
impl Canvas {
    pub fn new() -> Self {
        Canvas {
            response: None,
            painter: None,
            painter_area: Rect::ZERO,
            new_edge_start: None,
            comment_lines: CommentsGroup::new(),
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

    pub fn setup(&mut self, ctx: &eframe::egui::Context, ui: &mut Ui) {
        self.painter_area = ctx.available_rect().shrink(16.0);
        let size = self.painter_area.size();

        let (response, painter) = ui.allocate_painter(size, Sense::click_and_drag());
        painter.rect_filled(self.painter_area, 0.0, Color32::WHITE);

        self.response = Some(response);
        self.painter = Some(painter);
    }

    fn set_cursor_icon(&self, cursor_icon: egui::CursorIcon) {
        self.response().ctx.set_cursor_icon(cursor_icon);
    }
}

// nodes
impl Canvas {
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

    pub fn handle_node_draging(&mut self, graph: &mut Graph) {
        if let Some(mouse_pos) = self.response().interact_pointer_pos() {
            self.set_cursor_icon(egui::CursorIcon::Grabbing);

            if let Some(id) = graph.dragging() {
                let node = graph.nodes().get(&id).unwrap();
                graph.node_mut(&id).unwrap().position =
                    self.bounds_constraint_correction(node, mouse_pos);

                return;
            }

            for (id, node) in graph.nodes().iter() {
                if node.position.distance(mouse_pos) < node.radius {
                    graph.set_dragging(Some(*id));
                    break;
                }
            }
        } else {
            graph.set_dragging(None);
        }
    }

    pub fn handle_node_selection(&mut self, graph: &mut Graph) {
        if let Some(mouse_pos) = self.response().interact_pointer_pos() {
            for (id, node) in graph.nodes() {
                if node.position.distance(mouse_pos) < node.radius {
                    graph.set_selected_node_id(Some(*id));
                    break;
                }
            }
        }
    }

    pub fn draw_node(&self, node: &Node) {
        self.painter()
            .circle(node.position, node.radius, node.color, Stroke::NONE);

        let label_size = if node.label_size_matches_node_size {
            node.radius
        } else {
            node.label_size
        };

        self.painter().text(
            node.position,
            Align2::CENTER_CENTER,
            &node.label,
            FontId::new(label_size, FontFamily::Monospace),
            Color32::BLACK,
        );
    }

    pub fn draw_nodes(&mut self, graph: &Graph) {
        for node in graph.nodes().values() {
            self.draw_node(node);
        }
    }
}

// edges
impl Canvas {
    /// Return true if edge was created
    pub fn handle_edge_creation(&mut self, graph: &mut Graph) -> bool {
        // if Escape pressed => we dont' creating edge anymore
        if self
            .response()
            .ctx
            .input(|i| i.key_pressed(egui::Key::Escape))
        {
            self.new_edge_start = None;
            return false;
        }

        if !self.response().secondary_clicked() {
            return false;
        }

        if let Some(edge_start) = self.new_edge_start {
            let mouse_pos = self.response().interact_pointer_pos().unwrap();

            for (id, node) in graph.nodes() {
                if node.position.distance(mouse_pos) < node.radius {
                    graph.add_edge(Edge::new(edge_start, *id));
                    self.new_edge_start = None;

                    return true;
                }
            }
        }

        false
    }

    pub fn handle_setting_edge_start(&mut self, graph: &Graph) {
        if self.response().secondary_clicked() {
            let mouse_pos = self.response().interact_pointer_pos().unwrap();
            for (id, node) in graph.nodes() {
                if node.position.distance(mouse_pos) < node.radius {
                    self.new_edge_start = Some(*id);
                    break;
                }
            }
        }
    }

    pub fn draw_possible_edge(&mut self, graph: &Graph) {
        if let (Some(edge_start), Some(mouse_pos)) =
            (self.new_edge_start, self.response().hover_pos())
        {
            self.set_cursor_icon(egui::CursorIcon::PointingHand);
            let start_node = &graph.nodes()[&edge_start];

            if start_node.position.distance(mouse_pos) < start_node.radius {
                // draw loop
                let start_pos = start_node.position - Vec2::new(0.0, start_node.radius);
                let end_pos = start_node.position - Vec2::new(start_node.radius, 0.0);

                let control1 = start_pos - Vec2::new(0.0, CONTROL_OFFSET);
                let control2 = end_pos - Vec2::new(CONTROL_OFFSET, 0.0);

                self.painter().add(CubicBezierShape::from_points_stroke(
                    [start_pos, control1, control2, end_pos],
                    false,
                    Color32::TRANSPARENT,
                    Stroke::new(2.0, Color32::BLACK),
                ));
            } else {
                self.painter().line_segment(
                    [start_node.position, mouse_pos],
                    Stroke::new(2.0, Color32::BLACK),
                );
            }
        }
    }

    fn calculate_border_intersection(&self, node1: &Node, node2: &Node) -> (Pos2, Pos2) {
        let direction = (node2.position - node1.position).normalized();

        let start = node1.position + direction * node1.radius;
        let end = node2.position - direction * node2.radius;

        (start, end)
    }

    fn draw_edge_label(&self, ui: &mut Ui, edge: &Edge, start: Pos2, control: Pos2, end: Pos2) {
        let text = WidgetText::RichText(RichText::new(&edge.label).size(edge.label_size));
        let text_galley = text.into_galley(ui, None, f32::INFINITY, FontSelection::Default);
        let galley_size = text_galley.size();

        let direction = (end - start).normalized();
        let angle = if direction.x <= 0.0 {
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

    /// Rotate point on circle border (`border_pos`)
    /// relative to circle's center (`center_pos`) by `alpha` degree (in radians).
    fn rotate_border_point(&self, border_pos: Pos2, center_pos: Pos2, alpha: f32) -> Pos2 {
        let rotation = Rot2::from_angle(alpha);
        let translated = border_pos - center_pos;
        let rotated = rotation * translated;

        center_pos + rotated
    }

    fn draw_loop(&self, ui: &mut Ui, graph: &Graph, edge: &Edge, shift: f32) {
        let node = &graph.nodes()[&edge.start_id];

        let mut start = node.position - Vec2::new(0.0, node.radius);
        let mut end = node.position - Vec2::new(node.radius, 0.0);

        if edge.is_loop() {
            let rotation_angle = edge.loop_rotation_angle.to_radians();

            start = self.rotate_border_point(start, node.position, rotation_angle);
            end = self.rotate_border_point(end, node.position, rotation_angle);
        }

        let direction1 = (node.position - start).normalized();
        let direction2 = (node.position - end).normalized();

        let offset = CONTROL_OFFSET * (node.radius / 20.0) * (1.0 + shift);
        let control1 = start - direction1 * offset;
        let control2 = end - direction2 * offset;

        let curve = CubicBezierShape::from_points_stroke(
            [start, control1, control2, end],
            false,
            Color32::TRANSPARENT,
            Stroke::new(edge.width, edge.color),
        );
        let curve_control = curve.sample(0.5);
        self.painter().add(curve);

        if !edge.label.is_empty() {
            self.draw_edge_label(ui, edge, start, curve_control, end);
        }
    }

    fn draw_edge(&self, ui: &mut Ui, graph: &Graph, edge: &Edge, shift: f32) {
        let d = if edge.start_id < edge.end_id {
            -1.0
        } else {
            1.0
        };

        let (node_start, node_end) = (&graph.nodes()[&edge.start_id], &graph.nodes()[&edge.end_id]);
        let (start, end) = self.calculate_border_intersection(node_start, node_end);
        let start = self.rotate_border_point(start, node_start.position, DELTA_ANGLE * shift * d);
        let end = self.rotate_border_point(end, node_end.position, -DELTA_ANGLE * shift * d);

        let direction = if edge.start_id < edge.end_id {
            end - start
        } else {
            start - end
        }
        .normalized();

        let midpoint = Pos2::new((start.x + end.x) / 2.0, (start.y + end.y) / 2.0);
        let control = midpoint + direction.rot90() * shift * CONTROL_OFFSET;

        let curve = QuadraticBezierShape::from_points_stroke(
            [start, control, end],
            false,
            Color32::TRANSPARENT,
            Stroke::new(edge.width, edge.color),
        );
        let curve_control = curve.sample(0.5);
        self.painter().add(curve);

        if !edge.label.is_empty() {
            self.draw_edge_label(ui, edge, start, curve_control, end);
        }

        if edge.oriented {
            self.draw_arrow(control, end, edge.color, edge.width);
        }
    }

    pub fn draw_edges(&mut self, ui: &mut Ui, graph: &Graph) {
        let mut grouped_edges = HashMap::<(NodeId, NodeId), Vec<&Edge>>::new();

        for edge in graph.edges().values() {
            let group_key = if edge.start_id < edge.end_id {
                (edge.start_id, edge.end_id)
            } else {
                (edge.end_id, edge.start_id)
            };

            grouped_edges
                .entry(group_key)
                .and_modify(|v| v.push(edge))
                .or_insert(vec![edge]);
        }

        for ((start_id, end_id), edges) in grouped_edges {
            if start_id == end_id {
                // iterate over loops
                for (index, &edge) in edges.iter().enumerate() {
                    self.draw_loop(ui, &graph, edge, index as f32);
                }
            } else {
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
    pub fn handle_comment_draw(&mut self, stroke: Stroke) {
        self.set_cursor_icon(egui::CursorIcon::Cell);

        if self.comment_lines.is_empty() {
            self.comment_lines.insert(CommentLine::from(stroke));
        }

        let pointer_pos = self.response().interact_pointer_pos();

        let current_line = self.comment_lines.last_added_mut().unwrap();
        // update selected params
        if current_line.is_empty() {
            current_line.stroke = stroke;
        }

        if let Some(pointer_pos) = pointer_pos {
            if current_line.points.last() != Some(&pointer_pos) {
                current_line.points.push(pointer_pos);
            }
        } else if !current_line.is_empty() {
            self.comment_lines.insert(CommentLine::new());
        }
    }

    fn orientation(&self, a: Pos2, b: Pos2, c: Pos2) -> i32 {
        let value = (b.y - a.y) * (c.x - a.x) - (b.x - a.x) * (c.y - b.y);
        value.signum() as i32
    }

    fn on_segment(&self, a: Pos2, b: Pos2, c: Pos2) -> bool {
        b.x >= a.x.min(c.x) && b.x <= a.x.max(c.x) && b.y >= a.y.min(c.y) && b.y <= a.y.max(c.y)
    }

    fn segments_intersect(&self, a: Pos2, b: Pos2, c: Pos2, d: Pos2) -> bool {
        let o1 = self.orientation(a, b, c);
        let o2 = self.orientation(a, b, d);
        let o3 = self.orientation(c, d, a);
        let o4 = self.orientation(c, d, b);

        if o1 != o2 && o3 != o4 {
            return true;
        }

        (o1 == 0 && self.on_segment(a, c, b))
            || (o2 == 0 && self.on_segment(a, d, b))
            || (o3 == 0 && self.on_segment(c, a, d))
            || (o4 == 0 && self.on_segment(c, b, d))
    }

    fn is_intersect(&self, comment_line: &CommentLine, line: [Pos2; 2]) -> bool {
        let c = line[0];
        let d = line[1];

        for pair in comment_line.points.windows(2) {
            let a = pair[0];
            let b = pair[1];

            if self.segments_intersect(a, b, c, d) {
                return true;
            }
        }

        false
    }

    pub fn is_intersect_square(&self, comment_line: &CommentLine, square: Rect) -> bool {
        for point in &comment_line.points {
            if square.contains(*point) {
                return true;
            }

            if self.is_intersect(comment_line, [square.left_top(), square.right_top()])
                || self.is_intersect(comment_line, [square.right_top(), square.right_bottom()])
                || self.is_intersect(comment_line, [square.right_bottom(), square.left_bottom()])
                || self.is_intersect(comment_line, [square.left_bottom(), square.left_top()])
            {
                return true;
            }
        }

        false
    }

    pub fn handle_comment_erase(&mut self) {
        if self.response().hover_pos().is_none() {
            return;
        }

        self.set_cursor_icon(egui::CursorIcon::None);

        let square_center = self.response().hover_pos().unwrap();
        let square = Rect::from_center_size(square_center, Vec2::new(10.0, 10.0));

        self.painter()
            .rect_stroke(square, 0.0, Stroke::new(1.0, Color32::BLACK));

        if let Some(pointer_pos) = self.response().interact_pointer_pos() {
            let square = Rect::from_center_size(pointer_pos, Vec2::new(10.0, 10.0));

            self.painter()
                .rect_stroke(square, 0.0, Stroke::new(1.0, Color32::BLACK));

            let mut selected_line_id = None;
            for (id, line) in self.comment_lines.iter() {
                if self.is_intersect_square(line, square) {
                    selected_line_id = Some(id);
                    break;
                }
            }

            if let Some(id) = selected_line_id {
                self.comment_lines.remove(*id);
            }
        }
    }

    pub fn draw_comment_lines(&self) {
        let lines = self
            .comment_lines
            .iter()
            .filter(|(_, line)| line.len() >= 2)
            .map(|(_, line)| egui::Shape::line(line.points.clone(), line.stroke));

        self.painter().extend(lines);
    }
}
