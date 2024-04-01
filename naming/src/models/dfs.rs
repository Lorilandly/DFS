use super::{fs_node::FsNode, storage::Storage};
use crate::exception_return::ExceptionReturn;
use axum::response::IntoResponse;
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
        root.insert("/".into(), FsNode::new(true, Arc::default()));
        Dfs {
            storage: BTreeSet::default(),
            fs: root,
        }
    }
}

impl Dfs {
    pub fn insert(&mut self, path: &Path, is_dir: bool) -> Result<bool, impl IntoResponse> {
        if let Some(_) = self.fs.get(path) {
            Ok(false)
        } else if let Some(parent) = path.parent() {
            match self.is_dir(parent) {
                Ok(true) => {
                    self.fs.insert(
                        path.to_str().unwrap().into(),
                        FsNode::new(is_dir, Arc::default()),
                    );
                    self.fs
                        .get_mut(parent)
                        .unwrap()
                        .children
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
                Ok(false) => Err((
                    axum::http::StatusCode::BAD_REQUEST,
                    axum::Json(ExceptionReturn::new(
                        "FileNotFoundException",
                        "parent path is not a directory.",
                    )),
                )
                    .into_response()),
                Err(e) => Err(e.into_response()),
            }
        } else {
            Err((
                axum::http::StatusCode::BAD_REQUEST,
                axum::Json(ExceptionReturn::new(
                    "IllegalArgumentException",
                    "path cannot be empty.",
                )),
            )
                .into_response())
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

    pub fn insert_recursive(&mut self, path: &Path, is_dir: bool, storage: Arc<Storage>) -> bool {
        if !self.is_valid_path(path) {
            false
        } else if let Some(_) = self.fs.get(path) {
            if path == Path::new("/") {
                true
            } else {
                false
            }
        } else if let Some(parent) = path.parent() {
            self.insert_recursive(parent.into(), true, storage.clone());
            if let Some(parent_node) = self.fs.get_mut(parent) {
                parent_node
                    .children
                    .push(path.file_name().unwrap().to_str().unwrap().to_string());
            }
            self.fs.insert(
                path.to_str().unwrap().into(),
                FsNode {
                    is_dir: is_dir,
                    children: vec![],
                    storage: storage.clone(),
                },
            );
            true
        } else {
            panic!("This should not happen. Please check the code and check if root exists.")
        }
    }

    pub fn is_valid_path(&self, path: &Path) -> bool {
        path.is_absolute()
    }

    pub fn is_dir(&self, path: &Path) -> Result<bool, impl IntoResponse> {
        if !self.is_valid_path(path) {
            Err((
                axum::http::StatusCode::BAD_REQUEST,
                axum::Json(ExceptionReturn::new(
                    "IllegalArgumentException",
                    "path cannot be empty.",
                )),
            ))
        } else if let Some(node) = self.fs.get(path) {
            Ok(node.is_dir)
        } else {
            Err((
                axum::http::StatusCode::NOT_FOUND,
                axum::Json(ExceptionReturn::new(
                    "FileNotFoundException",
                    "the file/directory or parent directory does not exist.",
                )),
            ))
        }
    }

    pub fn list(&self, path: &Path) -> Result<Vec<String>, impl IntoResponse> {
        match self.is_dir(path) {
            Ok(true) => Ok(self.fs.get(path).unwrap().children.clone()),
            Ok(false) => Err((
                axum::http::StatusCode::BAD_REQUEST,
                axum::Json(ExceptionReturn::new(
                    "FileNotFoundException",
                    "path is not a directory.",
                )),
            )
                .into_response()),
            Err(e) => Err(e.into_response()),
        }
    }

    pub fn get_storage(&self, path: &Path) -> Result<Arc<Storage>, impl IntoResponse> {
        match self.is_dir(path) {
            Ok(true) => Err((
                axum::http::StatusCode::BAD_REQUEST,
                axum::Json(ExceptionReturn::new(
                    "FileNotFoundException",
                    "path is a directory.",
                )),
            )
                .into_response()),
            Ok(false) => Ok(self.fs.get(path).unwrap().storage.clone()),
            Err(e) => Err(e.into_response()),
        }
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
        let fs = Dfs::default();
        assert_eq!(fs.is_valid_path(Path::new("/")), true);
        assert_eq!(fs.is_valid_path(Path::new("a")), false);
        assert_eq!(fs.is_valid_path(Path::new("")), false);
    }
}
