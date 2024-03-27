use crate::handlers::exception_return::ExceptionReturn;
use axum::response::IntoResponse;
use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct Dfs {
    pub storage: HashSet<Arc<Storage>>,
    pub fs: Fs,
}

#[derive(Debug, PartialEq, Eq, Default, Hash)]
pub struct Storage {
    pub storage_ip: String,
    pub client_port: u16,
    pub command_port: u16,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Fs {
    root: BTreeMap<PathBuf, FSNode>,
}

impl Default for Fs {
    fn default() -> Self {
        let mut root = BTreeMap::default();
        root.insert(
            "/".into(),
            FSNode {
                is_dir: true,
                children: vec![],
                storage: Arc::default(),
            },
        );
        Fs { root }
    }
}

#[derive(Debug, PartialEq, Eq, Default, Hash)]
struct FSNode {
    is_dir: bool,
    children: Vec<String>,
    storage: Arc<Storage>,
}

impl Fs {
    pub fn insert(&mut self, path: &Path, is_dir: bool) -> Result<bool, impl IntoResponse> {
        if let Some(_) = self.root.get(path) {
            Ok(false)
        } else if let Some(parent) = path.parent() {
            match self.is_dir(parent) {
                Ok(true) => {
                    self.root.insert(
                        path.to_str().unwrap().into(),
                        FSNode {
                            is_dir: is_dir,
                            children: vec![],
                            storage: Arc::default(),
                        },
                    );
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
        } else if let Some(_) = self.root.get(path) {
            if path == Path::new("/") {
                true
            } else {
                false
            }
        } else if let Some(parent) = path.parent() {
            self.insert_recursive(parent.into(), true, storage.clone());
            if let Some(parent_node) = self.root.get_mut(parent) {
                parent_node
                    .children
                    .push(path.file_name().unwrap().to_str().unwrap().to_string());
            }
            self.root.insert(
                path.to_str().unwrap().into(),
                FSNode {
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
        } else if let Some(node) = self.root.get(path) {
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
            Ok(true) => Ok(self.root.get(path).unwrap().children.clone()),
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
            Ok(false) => Ok(self.root.get(path).unwrap().storage.clone()),
            Err(e) => Err(e.into_response()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_files() {
        let mut fs = Fs::default();
        let files = vec![PathBuf::from("/a"), PathBuf::from("/b")];
        assert_eq!(
            fs.insert_files(files, Arc::default()),
            Vec::<PathBuf>::new()
        );
        assert_eq!(fs.root.len(), 3);
        assert_eq!(fs.root.contains_key(Path::new("/")), true);
        assert_eq!(fs.root.contains_key(Path::new("/a")), true);
        assert_eq!(fs.root.contains_key(Path::new("/b")), true);
        let files2 = vec![PathBuf::from("a/b/c"), PathBuf::from("/a/c")];
        let delete = vec![PathBuf::from("a/b/c")];
        assert_eq!(fs.insert_files(files2, Arc::default()), delete);
    }

    #[test]
    fn test_valid_path() {
        let fs = Fs::default();
        assert_eq!(fs.is_valid_path(Path::new("/")), true);
        assert_eq!(fs.is_valid_path(Path::new("a")), false);
        assert_eq!(fs.is_valid_path(Path::new("")), false);
    }
}
