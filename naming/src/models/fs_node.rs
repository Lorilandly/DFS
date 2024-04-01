use super::storage::Storage;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Default, Hash)]
pub struct FsNode {
    pub is_dir: bool,
    pub children: Vec<String>,
    pub storage: Arc<Storage>,
}

impl FsNode {
    pub fn new(is_dir: bool, storage: Arc<Storage>) -> Self {
        Self {
            is_dir,
            children: vec![],
            storage,
        }
    }
}
