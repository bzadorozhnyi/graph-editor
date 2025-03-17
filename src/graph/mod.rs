pub mod edge;
pub mod node;

use std::collections::BTreeMap;
use std::collections::HashMap;

pub use edge::Edge;
use edge::EdgeId;
pub use node::Node;
pub use node::NodeId;

#[derive(Default)]
pub struct Graph {
    nodes: HashMap<NodeId, Node>,
    edges: BTreeMap<EdgeId, Edge>,
    selected_node_id: Option<NodeId>,
    selected_edge_id: Option<EdgeId>,
    dragging: Option<NodeId>,
    node_id_counter: usize,
    edge_id_counter: usize,
}

impl Graph {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn nodes(&self) -> &HashMap<NodeId, Node> {
        &self.nodes
    }

    pub fn node_mut(&mut self, id: &NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    pub fn edges(&self) -> &BTreeMap<EdgeId, Edge> {
        &self.edges
    }

    pub fn edge_mut(&mut self, id: &EdgeId) -> Option<&mut Edge> {
        self.edges.get_mut(id)
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

    pub fn dragging(&self) -> Option<NodeId> {
        self.dragging
    }

    pub fn set_dragging(&mut self, dragging: Option<NodeId>) {
        self.dragging = dragging;
    }

    pub fn selected_node_mut(&mut self) -> Option<&mut Node> {
        self.selected_node_id.map(|id| self.node_mut(&id).unwrap())
    }

    pub fn selected_edge_mut(&mut self) -> Option<&mut Edge> {
        self.selected_edge_id.map(|id| self.edge_mut(&id).unwrap())
    }

    pub fn selected_edge(&self) -> Option<&Edge> {
        match self.selected_edge_id {
            Some(id) => self.edges().get(&id),
            None => None,
        }
    }

    pub fn selected_edge_id(&mut self) -> &Option<EdgeId> {
        &self.selected_edge_id
    }

    pub fn set_selected_node_id(&mut self, node_id: Option<NodeId>) {
        self.selected_node_id = node_id;
    }

    pub fn set_selected_edge_id(&mut self, edge_id: Option<EdgeId>) {
        self.selected_edge_id = edge_id;
    }

    pub fn remove_selected_node(&mut self) {
        if let Some(selected_id) = self.selected_node_id {
            self.remove_node(selected_id);
        }
    }

    pub fn remove_selected_edge(&mut self) {
        if let Some(selected_id) = self.selected_edge_id {
            self.remove_edge(selected_id);
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

        self.nodes.remove(&id);
        self.edges.retain(|_, e| e.start_id != id && e.end_id != id);

        // meaning we removed edge when deleting edges connected to node
        // but must set selected id = None
        if self.selected_edge().is_none() {
            self.set_selected_edge_id(None);
        }
    }

    pub fn remove_edge(&mut self, id: EdgeId) {
        if let Some(selected_id) = self.selected_edge_id {
            if id == selected_id {
                self.selected_edge_id = None;
            }
        }

        self.edges.retain(|&edge_id, _| edge_id != id);
    }
}
