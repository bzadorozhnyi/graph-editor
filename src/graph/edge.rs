use super::NodeId;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EdgeId(pub usize);

#[derive(Debug)]
pub struct Edge {
    pub start_id: NodeId,
    pub end_id: NodeId,
    pub oriented: bool
}

impl Edge {
    pub fn new(start_index: NodeId, end_index: NodeId) -> Self {
        Self {
            start_id: start_index,
            end_id: end_index,
            oriented: true
        }
    }
}
