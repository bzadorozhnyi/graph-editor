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
    consts::{ARROW_HALF_ANGLE, ARROW_LEN_COEF, CONTROL_OFFSET, DELTA_ANGLE, MIN_NODE_RADIUS},
    graph::{Edge, Graph, Node, NodeId},
};

pub struct Canvas {
    response: Option<Response>,
    painter: Option<Painter>,
    painter_area: Rect,
    new_edge_start: Option<NodeId>,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            response: None,
            painter: None,
            painter_area: Rect::ZERO,
            new_edge_start: None,
        }
    }
}

// creation, setup and utils
impl Canvas {
    pub fn new() -> Self {
        Default::default()
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
    fn bounds_constraint_correction(&self, node: &Node, pointer_pos: Pos2) -> Pos2 {
        let new_x = if pointer_pos.x - node.radius < self.painter_area.min.x {
            self.painter_area.min.x + node.radius
        } else if pointer_pos.x + node.radius > self.painter_area.max.x {
            self.painter_area.max.x - node.radius
        } else {
            pointer_pos.x
        };

        let new_y = if pointer_pos.y - node.radius < self.painter_area.min.y {
            self.painter_area.min.y + node.radius
        } else if pointer_pos.y + node.radius > self.painter_area.max.y {
            self.painter_area.max.y - node.radius
        } else {
            pointer_pos.y
        };

        Pos2::new(new_x, new_y)
    }

    /// Handle node dragging.
    pub fn handle_node_draging(&mut self, graph: &mut Graph) {
        if let Some(pointer_pos) = self.response().interact_pointer_pos() {
            self.set_cursor_icon(egui::CursorIcon::Grabbing);

            // drag selected node to poiter pos
            if let Some(id) = graph.dragging_node() {
                let node = graph.nodes().get(&id).unwrap();
                graph.node_mut(&id).unwrap().position =
                    self.bounds_constraint_correction(node, pointer_pos);

                return;
            }

            // if pointer pos is the same as some node => mark node as dragging node
            for (id, node) in graph.nodes().iter() {
                if node.position.distance(pointer_pos) < node.radius {
                    graph.set_dragging_node(Some(*id));
                    break;
                }
            }
        } else {
            // any node is not node dragging
            graph.set_dragging_node(None);
        }
    }

    /// Mark one node selected if pointer position same as this node position.
    pub fn handle_node_selection(&mut self, graph: &mut Graph) {
        if let Some(pointer_pos) = self.response().interact_pointer_pos() {
            for (id, node) in graph.nodes() {
                if node.position.distance(pointer_pos) < node.radius {
                    graph.set_selected_node_id(Some(*id));
                    break;
                }
            }
        }
    }

    /// Draw node.
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

    /// Draw all nodes.
    pub fn draw_nodes(&mut self, graph: &Graph) {
        for node in graph.nodes().values() {
            self.draw_node(node);
        }
    }
}

// edges
impl Canvas {
    /// Handle edge creation.
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

        // if right mouse button was not clicked => ignore
        if !self.response().secondary_clicked() {
            return false;
        }

        if let Some(edge_start) = self.new_edge_start {
            let pointer_pos = self.response().interact_pointer_pos().unwrap();

            // if some node has same pos as pointer
            // then creating edge (edge_start; node)
            for (id, node) in graph.nodes() {
                if node.position.distance(pointer_pos) < node.radius {
                    graph.add_edge(Edge::new(edge_start, *id));
                    self.new_edge_start = None;

                    return true;
                }
            }
        }

        false
    }

    /// Handle setting start of edge (first selected node).
    /// (Edge is not created at this moment)
    pub fn handle_setting_edge_start(&mut self, graph: &Graph) {
        if self.response().secondary_clicked() {
            let pointer_pos = self.response().interact_pointer_pos().unwrap();

            // if some node has same pos as pointer
            // then set edge start as node id
            for (id, node) in graph.nodes() {
                if node.position.distance(pointer_pos) < node.radius {
                    self.new_edge_start = Some(*id);
                    break;
                }
            }
        }
    }

    /// Draw possible edge from new_edge_start node to pointer pos.
    pub fn draw_possible_edge(&mut self, graph: &Graph) {
        if let (Some(edge_start), Some(pointer_pos)) =
            (self.new_edge_start, self.response().hover_pos())
        {
            self.set_cursor_icon(egui::CursorIcon::PointingHand);
            let start_node = &graph.nodes()[&edge_start];

            if start_node.position.distance(pointer_pos) < start_node.radius {
                // draw loop
                // Start point is north of node
                let start_pos = start_node.position - Vec2::new(0.0, start_node.radius);
                // End point is west of node
                let end_pos = start_node.position - Vec2::new(start_node.radius, 0.0);

                // Calc offset based on node size
                let offset = CONTROL_OFFSET * (start_node.radius / MIN_NODE_RADIUS);

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

        let start = node1.position + direction * node1.radius;
        let end = node2.position - direction * node2.radius;

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

    /// Rotate point on circle border (`border_pos`)
    /// relative to circle's center (`center_pos`) by `alpha` degree (in radians).
    fn rotate_border_point(&self, border_pos: Pos2, center_pos: Pos2, alpha: f32) -> Pos2 {
        let rotation = Rot2::from_angle(alpha);
        let translated = border_pos - center_pos;
        let rotated = rotation * translated;

        center_pos + rotated
    }

    /// Draw loop edge
    fn draw_loop(&self, ui: &mut Ui, graph: &Graph, edge: &Edge, shift: f32) {
        let node = &graph.nodes()[&edge.start_id];

        let rotation_angle = edge.loop_rotation_angle.to_radians();

        // Calculate border points based on rotation angle.
        // Start point is north of node + rotation angle.
        let start = self.rotate_border_point(
            node.position - Vec2::new(0.0, node.radius),
            node.position,
            rotation_angle,
        );
        // End point is west of node + rotation angle.
        let end = self.rotate_border_point(
            node.position - Vec2::new(node.radius, 0.0),
            node.position,
            rotation_angle,
        );

        // Calc direction of vectors:
        // direction1: start -> node.center (node.position)
        let direction1 = (node.position - start).normalized();
        // direction2: end   -> node.center (node.position)
        let direction2 = (node.position - end).normalized();

        // Calc offset based on node size and shift (possible multiple loops)
        let offset = CONTROL_OFFSET * (node.radius / MIN_NODE_RADIUS) * (1.0 + shift);
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
        let edge_order = if edge.start_id < edge.end_id {
            -1.0
        } else {
            1.0
        };

        let (node_start, node_end) = (&graph.nodes()[&edge.start_id], &graph.nodes()[&edge.end_id]);
        let (start, end) = self.calculate_border_intersection(node_start, node_end);

        // Calc edge start and end to avoid edges overlaping
        // based on shift and edge_order
        let start =
            self.rotate_border_point(start, node_start.position, DELTA_ANGLE * shift * edge_order);
        let end =
            self.rotate_border_point(end, node_end.position, -DELTA_ANGLE * shift * edge_order);

        // Calc edge control for curve
        let direction = edge_order * (start - end).normalized();
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
    pub fn draw_edges(&mut self, ui: &mut Ui, graph: &Graph) {
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

    /// Orientation of a, b, c.
    /// - `0`  - collinear (lie on the same line)
    /// - `1`  - clockwise rotation
    /// - `-1` - counterclockwise rotation
    fn orientation(&self, a: Pos2, b: Pos2, c: Pos2) -> i32 {
        let value = (b.y - a.y) * (c.x - a.x) - (b.x - a.x) * (c.y - b.y);
        value.signum() as i32
    }

    /// Check if b is on (a; c) segment
    fn on_segment(&self, a: Pos2, b: Pos2, c: Pos2) -> bool {
        b.x >= a.x.min(c.x) && b.x <= a.x.max(c.x) && b.y >= a.y.min(c.y) && b.y <= a.y.max(c.y)
    }

    /// Check if segments (a; b) and (c; d) are intersecting
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

    /// Check if `comment_line` is intersect line
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

    /// Check if square is intersect `comment_line`
    pub fn is_intersect_square(&self, comment_line: &CommentLine, square: Rect) -> bool {
        let square_edges = [
            [square.left_top(), square.right_top()],
            [square.right_top(), square.right_bottom()],
            [square.right_bottom(), square.left_bottom()],
            [square.left_bottom(), square.left_top()],
        ];

        for point in &comment_line.points {
            if square.contains(*point)
                || square_edges
                    .iter()
                    .any(|edge| self.is_intersect(comment_line, *edge))
            {
                return true;
            }
        }

        false
    }

    pub fn handle_comment_erase(&mut self, comment_lines: &mut CommentsGroup) {
        if self.response().hover_pos().is_none() {
            return;
        }

        self.set_cursor_icon(egui::CursorIcon::None);

        // Create eraser square in hover_pos
        let square_center = self.response().hover_pos().unwrap();
        let hover_square = Rect::from_center_size(square_center, Vec2::new(10.0, 10.0));

        self.painter()
            .rect_stroke(hover_square, 0.0, Stroke::new(1.0, Color32::BLACK));

        if let Some(pointer_pos) = self.response().interact_pointer_pos() {
            // Create eraser square in pointer_pos
            let interact_square = Rect::from_center_size(pointer_pos, Vec2::new(10.0, 10.0));

            self.painter()
                .rect_stroke(interact_square, 0.0, Stroke::new(1.0, Color32::BLACK));

            // Find comment_line intersected by interact_square
            let mut selected_line_id = None;
            for (id, line) in comment_lines.iter() {
                if self.is_intersect_square(line, interact_square) {
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

    // Draw all comment lines.
    pub fn draw_comment_lines(&self, comment_lines: &CommentsGroup) {
        let lines = comment_lines
            .iter()
            .filter(|(_, line)| line.len() >= 2)
            .map(|(_, line)| egui::Shape::line(line.points.clone(), line.stroke));

        self.painter().extend(lines);
    }
}
