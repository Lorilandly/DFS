use super::{fs_node::FsNode, storage::Storage};
use crate::exception_return::ExceptionReturn;
use axum::{http::StatusCode, Json};
use parking_lot::lock_api::RawRwLock;
use std::{
    collections::{BTreeMap, BTreeSet},
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
        root.insert("/".into(), FsNode::new(true, vec![], Arc::default()));
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
    pub fn insert(&mut self, path: &Path, is_dir: bool) -> Result<bool> {
        if let Some(_) = self.fs.get(path) {
            Ok(false)
        } else if let Some(parent) = path.parent() {
            match self.is_dir(parent)? {
                true => {
                    self.fs.insert(
                        path.to_str().unwrap().into(),
                        FsNode::new(is_dir, vec![], Arc::default()),
                    );
                    self.fs
                        .get(parent)
                        .unwrap()
                        .children
                        .write()
                        .push(path.file_name().unwrap().to_str().unwrap().to_string());
                    if let Ok(false) = self.is_dir(path) {
                        let client = reqwest::blocking::Client::new();
                        let command_port = self.storage.first().unwrap().command_port;
                        let _res = client
                            .post(format!("http://localhost:{}/storage_create", command_port))
                            .body(format!("{{\"path\": \"{}\"}}", path.to_str().unwrap()))
                            .send()
                            .unwrap();
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

    pub fn insert_files(&mut self, files: Vec<PathBuf>, storage: Arc<Storage>) -> Vec<PathBuf> {
        let mut existed_files = vec![];
        for file in files {
            if !self.insert_recursive(&file, false, storage.clone()) {
                existed_files.push(file);
            }
        }
        existed_files
    }

    fn insert_recursive(&mut self, path: &Path, is_dir: bool, storage: Arc<Storage>) -> bool {
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
                FsNode::new(is_dir, vec![], storage.clone()),
            );
            while let Some(parent) = chld.parent() {
                let file_name = chld.file_name().unwrap().to_str().unwrap().to_string();
                if let Some(parent_node) = self.fs.get(parent) {
                    parent_node.children.write().push(file_name.clone());
                    break;
                }
                self.fs.insert(
                    parent.to_str().unwrap().into(),
                    FsNode::new(true, vec![file_name], storage.clone()),
                );
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
            true => Ok(self.fs.get(path).unwrap().children.read().clone()),
            false => Err((
                StatusCode::BAD_REQUEST,
                axum::Json(ExceptionReturn::new(
                    "FileNotFoundException",
                    "path is not a directory.",
                )),
            )),
        }
    }

    pub fn get_storage(&self, path: &Path) -> Result<Arc<Storage>> {
        match self.is_dir(path)? {
            true => Err((
                StatusCode::BAD_REQUEST,
                axum::Json(ExceptionReturn::new(
                    "FileNotFoundException",
                    "path is a directory.",
                )),
            )),
            false => Ok(self.fs.get(path).unwrap().storage.clone()),
        }
    }

    pub async fn lock(&self, path: &Path, exclusive: bool) -> Result<()> {
        self.is_dir(path)?;
        if let Some(node) = self.fs.get(path) {
            tracing::info!("lock path: {:?}", path);
            let files_to_lock = Self::get_ancestors(path);
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
            tracing::info!("lock path: {:?} success", path);
        }
        Ok(())
    }

    pub fn unlock(&self, path: &Path, exclusive: bool) -> Result<()> {
        // TODO: check if the file node is locked explicitly using `lock()`
        if let Some(node) = self.fs.get(path) {
            // try unlocking the file node
            // check if the file node is locked
            if exclusive && node.children.is_locked_exclusive() {
                tracing::info!("unlock path: {:?}", path);
                unsafe { node.children.raw().unlock_exclusive() };
                tracing::info!("unlock path: {:?} success", path);
            } else if node.children.is_locked() {
                tracing::info!("unlock path: {:?}", path);
                unsafe { node.children.raw().unlock_shared() };
                tracing::info!("unlock path: {:?} success", path);
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

    #[test]
    fn test_insert_files() {
        let mut fs = Dfs::default();
        let files = vec![PathBuf::from("/a"), PathBuf::from("/b")];
        assert_eq!(
            fs.insert_files(files, Arc::default()),
            Vec::<PathBuf>::new()
        );
        assert_eq!(fs.fs.len(), 3);
        assert_eq!(fs.fs.contains_key(Path::new("/")), true);
        assert_eq!(fs.fs.contains_key(Path::new("/a")), true);
        assert_eq!(fs.fs.contains_key(Path::new("/b")), true);
        let files2 = vec![PathBuf::from("a/b/c"), PathBuf::from("/a/c")];
        let delete = vec![PathBuf::from("a/b/c")];
        assert_eq!(fs.insert_files(files2, Arc::default()), delete);
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
