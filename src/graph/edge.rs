use super::NodeId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EdgeId(pub usize);

#[derive(Debug)]
pub struct Edge {
    pub start_id: NodeId,
    pub end_id: NodeId,
    pub oriented: bool,
    pub label: String,
    pub padding_x: f32,
    pub padding_y: f32
}

impl Edge {
    pub fn new(start_index: NodeId, end_index: NodeId) -> Self {
        Self {
            start_id: start_index,
            end_id: end_index,
            oriented: true,
            label: String::new(),
            padding_x: 0.0,
            padding_y: 0.0,
        }
    }
}
