//! File system node model
use super::storage::Storage;
use crate::requests;
use futures::future::join_all;
use parking_lot::RwLock;
use std::{
    collections::{BTreeSet, HashSet},
    path::Path,
    sync::{atomic::AtomicI32, Arc},
};
use tokio::sync::Mutex;

#[derive(Debug, Default)]
pub struct FsNode {
    pub is_dir: bool,
    /// When the node is a directory, children is a set of file names under the directory.
    pub children: RwLock<HashSet<String>>,
    /// storages is a set of storage servers that the node is stored in.
    storages: Mutex<BTreeSet<Arc<Storage>>>,
    /// access_count is the number of times the node is accessed (locked).
    pub access_count: AtomicI32,
}

impl FsNode {
    /// Creates a new FsNode object
    pub fn new(is_dir: bool, child: HashSet<String>, storage: Arc<Storage>) -> Self {
        let storages = Mutex::new(BTreeSet::from([storage]));
        Self {
            is_dir,
            children: RwLock::new(child),
            storages,
            access_count: AtomicI32::new(0),
        }
    }

    /// Add a new storage to the node
    pub async fn add_storage(&self, storage: Arc<Storage>) {
        let mut storages = self.storages.lock().await;
        storages.insert(storage);
    }

    /// Replicate to a new storage for the node
    ///
    /// This function compare the given storages with the current storages, if there
    /// are any storage in the given storages that are not in the current storages,
    /// the function will pick one storage and add to the current storages.
    ///
    /// # Panics
    ///
    /// Panics if no storage is currently associated with the node.
    pub async fn replicate_storage(&self, storage_pool: &BTreeSet<Arc<Storage>>, path: &Path) {
        let mut storages = self.storages.lock().await;
        if let Some(storage) = storage_pool.difference(&storages).next().cloned() {
            let current = storages.first().unwrap().clone();
            storages.insert(storage.clone());
            requests::storage_copy(&storage, &current, path)
                .await
                .expect("Failed to copy storage: Request returned error.");
        }
    }

    /// Remove all storage except for one
    ///
    /// This function returns the storages that are removed
    pub async fn dereplicate_storage(&self, path: &Path) {
        let mut storages = self.storages.lock().await;
        let threads = storages
            .iter()
            .skip(1)
            .map(|storage| requests::storage_delete(&storage, path))
            .collect::<Vec<_>>();
        join_all(threads).await;
        while storages.len() > 1 {
            storages.pop_last().unwrap();
        }
    }

    /// Remove one storage at a time from the node
    ///
    /// This function is used before deleting the node
    pub async fn remove_storage(&self, path: &Path) {
        let mut storages = self.storages.lock().await;
        let threads = storages
            .iter()
            .map(|storage| requests::storage_delete(&storage, path))
            .collect::<Vec<_>>();
        join_all(threads).await;
        storages.clear();
    }

    /// Get a storage from the node
    pub async fn get_storage(&self) -> Option<Arc<Storage>> {
        // get a random storage from storages
        let storages = self.storages.lock().await;
        storages
            .iter()
            .skip(rand::random::<usize>() % storages.len())
            .next()
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_storage() {
        let fs_node = FsNode::new(true, HashSet::new(), Arc::new(Storage::default()));
        // fs_node.storages.lock().await.insert(Arc::new(Storage::default()));
        assert!(fs_node.get_storage().await.is_some());
    }
}
