pub mod edge;
pub mod node;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::sync::Mutex;

pub use edge::Edge;
use edge::EdgeId;
use eframe::egui::pos2;
pub use node::shape::NodeShape;
pub use node::Node;
pub use node::NodeId;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::Deserialize;
use serde::Serialize;

use crate::consts::{
    DEFAULT_NODE_X_POSITION, DEFAULT_NODE_Y_POSITION, MAX_NODE_SIZE, MIN_NODE_SIZE,
};
use crate::error::GraphEditorError;

static RNG: LazyLock<Mutex<StdRng>> = LazyLock::new(|| Mutex::new(StdRng::seed_from_u64(0)));

#[derive(Default, Serialize, Deserialize)]
pub struct Graph {
    nodes: HashMap<NodeId, Node>,
    edges: BTreeMap<EdgeId, Edge>,
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

    pub fn node(&self, id: &NodeId) -> Option<&Node> {
        self.nodes.get(id)
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

    pub fn add_node(&mut self) {
        self.node_id_counter += 1;
        let node_id = self.node_id_counter;

        let position = pos2(
            DEFAULT_NODE_X_POSITION + self.random_node_position_offset(),
            DEFAULT_NODE_Y_POSITION + self.random_node_position_offset(),
        );
        let new_node = Node::new(node_id.to_string(), position);

        self.nodes.insert(NodeId(node_id), new_node);
    }

    fn random_node_position_offset(&self) -> f32 {
        let mut rng = RNG.lock().unwrap();
        rng.random_range(2.0 * MIN_NODE_SIZE..=2.0 * MAX_NODE_SIZE)
    }

    pub fn add_edge(&mut self, start_id: NodeId, end_id: NodeId) {
        self.edge_id_counter += 1;
        let edge_id = self.edge_id_counter;

        let new_edge = Edge::new(start_id, end_id);

        self.edges.insert(EdgeId(edge_id), new_edge);
    }

    pub fn remove_node(&mut self, id: NodeId) {
        self.nodes.remove(&id);
        self.edges.retain(|_, e| e.start_id != id && e.end_id != id);
    }

    pub fn remove_edge(&mut self, id: EdgeId) {
        self.edges.retain(|&edge_id, _| edge_id != id);
    }

    pub fn save_to_file(&self, file_path: &PathBuf) -> Result<(), GraphEditorError> {
        let graph_json = serde_json::to_string_pretty(&self);

        match graph_json {
            Ok(value) => {
                let mut file =
                    File::create(file_path).map_err(|_| GraphEditorError::FailedSaveFile)?;
                file.write_all(value.as_bytes())
                    .map_err(|_| GraphEditorError::FailedSaveFile)?;
            }
            Err(_) => {
                return Err(GraphEditorError::FailedSaveFile);
            }
        }

        Ok(())
    }
    
    pub fn edge(&self, id: &EdgeId) -> Option<&Edge> {
        self.edges.get(id)
    }
}

impl TryFrom<&PathBuf> for Graph {
    type Error = GraphEditorError;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        let file: File = File::open(value).map_err(|_| GraphEditorError::FailedOpenFile)?;
        let reader = BufReader::new(file);

        serde_json::from_reader(reader).map_err(|_| GraphEditorError::FailedOpenFile)
    }
}
