pub mod edge;
pub mod node;

use std::collections::BTreeMap;
use std::collections::HashMap;

pub use edge::Edge;
use edge::EdgeId;
use eframe::egui::Painter;
pub use node::Node;
pub use node::NodeId;

pub struct Graph {
    nodes: HashMap<NodeId, Node>,
    edges: BTreeMap<EdgeId, Edge>,
    selected_node_id: Option<NodeId>,
    dragging: Option<NodeId>,
    node_id_counter: usize,
    edge_id_counter: usize,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: BTreeMap::new(),
            selected_node_id: None,
            dragging: None,
            node_id_counter: 0,
            edge_id_counter: 0
        }
    }

    pub fn nodes(&self) -> &HashMap<NodeId, Node> {
        &self.nodes
    }

    pub fn nodes_mut(&mut self) -> &mut HashMap<NodeId, Node> {
        &mut self.nodes
    }

    pub fn edges(&self) -> &BTreeMap<EdgeId, Edge> {
        &self.edges
    }

    pub fn edges_mut(&mut self) -> &mut BTreeMap<EdgeId, Edge> {
        &mut self.edges
    }

    pub fn add_node(&mut self, node: Node) {
        let node_id = self.node_id_counter;
        self.node_id_counter += 1;

        self.nodes.insert(NodeId(node_id), node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        let edge_id = self.edge_id_counter;
        self.edge_id_counter += 1;

        self.edges.insert(EdgeId(edge_id), edge);
    }

    pub fn draw_nodes(&self, painter: &Painter) {
        for node in self.nodes.values() {
            node.draw(painter);
        }
    }

    pub fn dragging(&self) -> Option<NodeId> {
        self.dragging
    }

    pub fn set_dragging(&mut self, dragging: Option<NodeId>) {
        self.dragging = dragging;
    }

    pub fn selected_node_mut(&mut self) -> Option<&mut Node> {
        self.selected_node_id
            .map(|id| self.nodes_mut().get_mut(&id).unwrap())
    }

    pub fn set_selected_node_id(&mut self, selected_node_id: Option<NodeId>) {
        self.selected_node_id = selected_node_id;
    }

    pub fn remove_selected(&mut self) {
        if let Some(selected_id) = self.selected_node_id {
            self.remove_node(selected_id);
        }
    }

    pub fn remove_node(&mut self, id: NodeId) {
        if let Some(selected_id) = self.selected_node_id {
            if id == selected_id {
                self.selected_node_id = None;
            }
        }

        if let Some(dragging_id) = self.dragging {
            if id == dragging_id {
                self.dragging = None;
            }
        }

        self.nodes_mut().remove(&id);
        self.edges_mut()
            .retain(|_, e| e.start_id != id && e.end_id != id);
    }
}
