use super::storage::Storage;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Default)]
pub struct FsNode {
    pub is_dir: bool,
    pub children: RwLock<Vec<String>>,
    pub storage: Arc<Storage>,
}

impl FsNode {
    pub fn new(is_dir: bool, child: Vec<String>, storage: Arc<Storage>) -> Self {
        Self {
            is_dir,
            children: RwLock::new(child),
            storage,
        }
    }
}
