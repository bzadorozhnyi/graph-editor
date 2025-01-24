#[derive(Debug)]
pub struct Edge {
    pub start_index: usize,
    pub end_index: usize,
}

impl Edge {
    pub fn new(start_index: usize, end_index: usize) -> Self {
        Self {
            start_index,
            end_index,
        }
    }
}
