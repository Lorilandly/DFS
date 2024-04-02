use super::{fs_node::FsNode, storage::Storage};
use crate::exception_return::ExceptionReturn;
use axum::{http::StatusCode, Json};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
    sync::Arc,
};

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

impl Dfs {
    pub async fn insert(
        &mut self,
        path: &Path,
        is_dir: bool,
    ) -> Result<bool, (StatusCode, Json<ExceptionReturn>)> {
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
                        .await
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
                FsNode::new(is_dir, vec![], storage.clone()),
            );
            while let Some(parent) = chld.parent() {
                let file_name = chld.file_name().unwrap().to_str().unwrap().to_string();
                if let Some(parent_node) = self.fs.get(parent) {
                    parent_node.children.write().await.push(file_name.clone());
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

    pub fn is_dir(&self, path: &Path) -> Result<bool, (StatusCode, Json<ExceptionReturn>)> {
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

    pub async fn list(
        &self,
        path: &Path,
    ) -> Result<Vec<String>, (StatusCode, Json<ExceptionReturn>)> {
        match self.is_dir(path)? {
            true => Ok(self.fs.get(path).unwrap().children.read().await.clone()),
            false => Err((
                StatusCode::BAD_REQUEST,
                axum::Json(ExceptionReturn::new(
                    "FileNotFoundException",
                    "path is not a directory.",
                )),
            )),
        }
    }

    pub fn get_storage(
        &self,
        path: &Path,
    ) -> Result<Arc<Storage>, (StatusCode, Json<ExceptionReturn>)> {
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
}
