use super::{fs_node::FsNode, storage::Storage};
use crate::exception_return::ExceptionReturn;
use axum::{http::StatusCode, Json};
use parking_lot::lock_api::RawRwLock;
use std::{
    collections::{BTreeMap, BTreeSet, HashSet, VecDeque},
    path::{Path, PathBuf},
    sync::Arc,
};

type Result<T> = std::result::Result<T, (StatusCode, Json<ExceptionReturn>)>;

#[derive(Debug)]
pub struct Dfs {
    pub storage: BTreeSet<Arc<Storage>>,
    fs: BTreeMap<PathBuf, FsNode>,
}

impl Default for Dfs {
    fn default() -> Self {
        let mut root = BTreeMap::default();
        root.insert(
            "/".into(),
            FsNode::new(true, HashSet::new(), Arc::default()),
        );
        Dfs {
            storage: BTreeSet::default(),
            fs: root,
        }
    }
}

// Issue: each child node is locked with a synchroneous lock. If there
//   is deadlock on unlock that is not wrapped inside a tokio block_in_place
//   block, the program will hang.
impl Dfs {
    fn random_storage(&self) -> Result<Arc<Storage>> {
        // get a random storage from storages
        match self
            .storage
            .iter()
            .skip(rand::random::<usize>() % self.storage.len())
            .next()
        {
            Some(storage) => Ok(storage.clone()),
            None => Err((
                StatusCode::NOT_FOUND,
                Json(ExceptionReturn::new(
                    "NotFoundException",
                    "No storage available",
                )),
            )),
        }
    }

    pub async fn insert(&mut self, path: &Path, is_dir: bool) -> Result<bool> {
        if let Some(_) = self.fs.get(path) {
            Ok(false)
        } else if let Some(parent) = path.parent() {
            match self.is_dir(parent)? {
                true => {
                    let storage = self.random_storage()?;
                    self.fs.insert(
                        path.into(),
                        FsNode::new(is_dir, HashSet::new(), storage.clone()),
                    );
                    self.fs
                        .get(parent)
                        .unwrap()
                        .children
                        .write()
                        .insert(path.file_name().unwrap().to_str().unwrap().to_string());
                    if !is_dir {
                        let _ = crate::requests::storage_create(&storage, path).await;
                    }
                    Ok(true)
                }
                false => Err((
                    StatusCode::BAD_REQUEST,
                    axum::Json(ExceptionReturn::new(
                        "FileNotFoundException",
                        "parent path is not a directory.",
                    )),
                )),
            }
        } else {
            Err((
                StatusCode::BAD_REQUEST,
                axum::Json(ExceptionReturn::new(
                    "IllegalArgumentException",
                    "path cannot be empty.",
                )),
            ))
        }
    }

    pub async fn insert_files(
        &mut self,
        files: Vec<PathBuf>,
        storage: Arc<Storage>,
    ) -> Vec<PathBuf> {
        let mut existed_files = vec![];
        for file in files {
            if !self.insert_recursive(&file, false, storage.clone()).await {
                existed_files.push(file);
            }
        }
        existed_files
    }

    async fn insert_recursive(&mut self, path: &Path, is_dir: bool, storage: Arc<Storage>) -> bool {
        if !Self::is_valid_path(path) {
            false
        } else if let Some(_) = self.fs.get(path) {
            if path == Path::new("/") {
                true
            } else {
                false
            }
        } else {
            let mut chld = path;
            self.fs.insert(
                chld.to_str().unwrap().into(),
                FsNode::new(is_dir, HashSet::new(), storage.clone()),
            );
            while let Some(parent) = chld.parent() {
                let file_name = chld.file_name().unwrap().to_str().unwrap().to_string();
                if let Some(parent_node) = self.fs.get(parent) {
                    parent_node.children.write().insert(file_name.clone());
                    parent_node.add_storage(storage.clone()).await;
                } else {
                    self.fs.insert(
                        parent.to_str().unwrap().into(),
                        FsNode::new(true, HashSet::from([file_name]), storage.clone()),
                    );
                }
                chld = parent;
            }
            true
        }
    }

    pub fn is_valid_path(path: &Path) -> bool {
        path.is_absolute()
    }

    fn get_ancestors(path: &Path) -> Vec<PathBuf> {
        path.ancestors().skip(1).map(|p| p.to_path_buf()).collect()
    }

    pub fn is_dir(&self, path: &Path) -> Result<bool> {
        if !Self::is_valid_path(path) {
            Err((
                StatusCode::BAD_REQUEST,
                axum::Json(ExceptionReturn::new(
                    "IllegalArgumentException",
                    "path cannot be empty.",
                )),
            ))
        } else if let Some(node) = self.fs.get(path) {
            Ok(node.is_dir)
        } else {
            Err((
                StatusCode::NOT_FOUND,
                axum::Json(ExceptionReturn::new(
                    "FileNotFoundException",
                    "the file/directory or parent directory does not exist.",
                )),
            ))
        }
    }

    pub fn list(&self, path: &Path) -> Result<Vec<String>> {
        match self.is_dir(path)? {
            true => Ok(self
                .fs
                .get(path)
                .unwrap()
                .children
                .read()
                .clone()
                .into_iter()
                .collect()),
            false => Err((
                StatusCode::BAD_REQUEST,
                axum::Json(ExceptionReturn::new(
                    "FileNotFoundException",
                    "path is not a directory.",
                )),
            )),
        }
    }

    pub async fn get_storage(&self, path: &Path) -> Result<Arc<Storage>> {
        match self.is_dir(path)? {
            true => Err((
                StatusCode::BAD_REQUEST,
                axum::Json(ExceptionReturn::new(
                    "FileNotFoundException",
                    "path is a directory.",
                )),
            )),
            false => Ok(self.fs.get(path).unwrap().get_storage().await.unwrap()),
        }
    }

    pub async fn delete(&mut self, path: &Path) -> Result<bool> {
        self.is_dir(path)?;
        // del parent children
        // add self to del queue
        // loop del queue:
        //  add self children to del queue
        //  del self
        self.fs
            .get(path.parent().unwrap())
            .unwrap()
            .children
            .write()
            .remove(path.file_name().unwrap().to_str().unwrap());
        let mut queue = VecDeque::from([path.to_path_buf()]);
        self.fs.get(path).unwrap().remove_storage(&path).await;
        while let Some(path) = queue.pop_front() {
            let node = self.fs.remove(&path).unwrap();
            node.children.write().iter().for_each(|child| {
                queue.push_back(path.join(child));
            });
        }
        Ok(true)
    }

    pub async fn lock(&self, path: &Path, exclusive: bool) -> Result<()> {
        self.is_dir(path)?;
        if let Some(node) = self.fs.get(path) {
            let files_to_lock = Self::get_ancestors(path);
            // if lock for many times, replicate
            let count = node
                .access_count
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if !node.is_dir {
                if exclusive {
                    node.dereplicate_storage(path).await;
                } else if count % 5 == 0 {
                    // replicate
                    node.replicate_storage(&self.storage, path).await;
                }
            }
            tokio::task::block_in_place(|| unsafe {
                if exclusive {
                    node.children.raw().lock_exclusive();
                } else {
                    node.children.raw().lock_shared();
                }
                files_to_lock.iter().for_each(|p| {
                    if let Some(node) = self.fs.get(p) {
                        node.children.raw().lock_shared();
                    }
                });
            });
        }
        Ok(())
    }

    pub fn unlock(&self, path: &Path, exclusive: bool) -> Result<()> {
        // TODO: check if the file node is locked explicitly using `lock()`
        if let Some(node) = self.fs.get(path) {
            // try unlocking the file node
            // check if the file node is locked
            if exclusive && node.children.is_locked_exclusive() {
                unsafe { node.children.raw().unlock_exclusive() };
            } else if node.children.is_locked() {
                unsafe { node.children.raw().unlock_shared() };
            } else {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(ExceptionReturn::new(
                        "IllegalArgumentException",
                        "Attempt to unlock an unlocked file",
                    )),
                ));
            }
        } else {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ExceptionReturn::new(
                    "IllegalArgumentException",
                    "Illegal path",
                )),
            ));
        }
        let files_to_unlock = Self::get_ancestors(path);
        files_to_unlock.iter().for_each(|p| {
            if let Some(node) = self.fs.get(p) {
                unsafe { node.children.raw().unlock_shared() };
            }
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_insert_files() {
        let mut fs = Dfs::default();
        let files = vec![PathBuf::from("/a"), PathBuf::from("/b")];
        assert_eq!(
            fs.insert_files(files, Arc::default()).await,
            Vec::<PathBuf>::new()
        );
        assert_eq!(fs.fs.len(), 3);
        assert_eq!(fs.fs.contains_key(Path::new("/")), true);
        assert_eq!(fs.fs.contains_key(Path::new("/a")), true);
        assert_eq!(fs.fs.contains_key(Path::new("/b")), true);
        let files2 = vec![PathBuf::from("a/b/c"), PathBuf::from("/a/c")];
        let delete = vec![PathBuf::from("a/b/c")];
        assert_eq!(fs.insert_files(files2, Arc::default()).await, delete);
    }

    #[test]
    fn test_valid_path() {
        assert_eq!(Dfs::is_valid_path(Path::new("/")), true);
        assert_eq!(Dfs::is_valid_path(Path::new("a")), false);
        assert_eq!(Dfs::is_valid_path(Path::new("")), false);
    }

    #[test]
    fn test_get_ansestors() {
        let path = Path::new("/a/b/c");
        let ancestors = Dfs::get_ancestors(path);
        assert_eq!(
            ancestors,
            vec![
                PathBuf::from("/a/b"),
                PathBuf::from("/a"),
                PathBuf::from("/")
            ]
        );
    }
}
