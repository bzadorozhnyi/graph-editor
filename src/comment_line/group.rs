use std::collections::HashMap;

use crate::comment_line::CommentLine;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct CommentId(usize);

#[derive(Default)]
pub struct CommentsGroup {
    data: HashMap<CommentId, CommentLine>,
    comment_id_counter: usize,
}

impl CommentsGroup {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, line: CommentLine) {
        self.comment_id_counter += 1;
        let id = self.comment_id_counter;

        self.data.insert(CommentId(id), line);
    }

    pub fn remove(&mut self, id: CommentId) -> Option<CommentLine> {
        self.data.remove(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&CommentId, &CommentLine)> {
        self.data.iter()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn last_added_mut(&mut self) -> Option<&mut CommentLine> {
        let last_id = self.comment_id_counter;

        self.data.get_mut(&CommentId(last_id))
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.comment_id_counter = 0;
    }
}
