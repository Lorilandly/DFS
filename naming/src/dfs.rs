use crate::handlers::exception_return::ExceptionReturn;
use axum::response::IntoResponse;
use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct Dfs {
    pub storage: HashSet<Storage>,
    pub fs: FS,
}

#[derive(Debug, PartialEq, Eq, Default, Hash)]
pub struct Storage {
    pub storage_ip: String,
    pub client_port: u16,
    pub command_port: u16,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FS {
    root: BTreeMap<PathBuf, FSNode>,
}

impl Default for FS {
    fn default() -> Self {
        let mut root = BTreeMap::default();
        root.insert(
            "/".into(),
            FSNode {
                is_dir: true,
                children: vec![],
            },
        );
        FS { root }
    }
}

#[derive(Debug, PartialEq, Eq, Default, Hash)]
struct FSNode {
    is_dir: bool,
    children: Vec<String>,
}

impl FS {
    pub fn insert_files(&mut self, files: Vec<PathBuf>) -> Vec<PathBuf> {
        let mut existed_files = vec![];
        for file in files {
            if !self.insert(&file, false) {
                existed_files.push(file);
            }
        }
        existed_files
    }

    pub fn insert(&mut self, path: &Path, is_dir: bool) -> bool {
        if !self.is_valid_path(path) {
            false
        } else if let Some(_) = self.root.get(path) {
            if path == Path::new("/") {
                true
            } else {
                false
            }
        } else if let Some(parent) = path.parent() {
            self.insert(parent.into(), true);
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
                },
            );
            true
        } else {
            panic!("This should not happen. Please check the code and check if root exists.")
        }
    }

    fn is_valid_path(&self, path: &Path) -> bool {
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
}

#[cfg(test)]
mod tests {
    use crate::handlers::delete;

    use super::*;

    #[test]
    fn test_insert_files() {
        let mut fs = FS::default();
        let files = vec![PathBuf::from("/a"), PathBuf::from("/b")];
        assert_eq!(fs.insert_files(files), Vec::<PathBuf>::new());
        assert_eq!(fs.root.len(), 3);
        assert_eq!(fs.root.contains_key(Path::new("/")), true);
        assert_eq!(fs.root.contains_key(Path::new("/a")), true);
        assert_eq!(fs.root.contains_key(Path::new("/b")), true);
        let files2 = vec![PathBuf::from("a/b/c"), PathBuf::from("/a/c")];
        let delete = vec![PathBuf::from("a/b/c")];
        assert_eq!(fs.insert_files(files2), delete);
    }

    #[test]
    fn test_valid_path() {
        let fs = FS::default();
        assert_eq!(fs.is_valid_path(Path::new("/")), true);
        assert_eq!(fs.is_valid_path(Path::new("a")), false);
        assert_eq!(fs.is_valid_path(Path::new("")), false);
    }
}
