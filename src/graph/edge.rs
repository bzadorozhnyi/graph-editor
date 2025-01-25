use super::NodeId;

#[derive(Debug)]
pub struct Edge {
    pub start_index: NodeId,
    pub end_index: NodeId,
    pub oriented: bool
}

impl Edge {
    pub fn new(start_index: NodeId, end_index: NodeId) -> Self {
        Self {
            start_index,
            end_index,
            oriented: true
        }
    }
}
