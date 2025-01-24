pub mod edge;
pub mod node;

pub use edge::Edge;
use eframe::egui::Painter;
pub use node::Node;

pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    selected_node_index: Option<usize>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            selected_node_index: None,
        }
    }

    pub fn nodes(&self) -> &Vec<Node> {
        &self.nodes
    }

    pub fn nodes_mut(&mut self) -> &mut Vec<Node> {
        &mut self.nodes
    }

    pub fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    pub fn edges_mut(&mut self) -> &mut Vec<Edge> {
        &mut self.edges
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn draw_nodes(&self, painter: &Painter) {
        for node in &self.nodes {
            node.draw(painter);
        }
    }

    pub fn selected_node_mut(&mut self) -> Option<&mut Node> {
        self.selected_node_index
            .map(|index| &mut self.nodes_mut()[index])
    }

    pub fn set_selected_node_index(&mut self, selected_node_index: Option<usize>) {
        self.selected_node_index = selected_node_index;
    }
}
