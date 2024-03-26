use std::collections::{BTreeMap, HashSet};
use std::path::Path;

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

#[derive(Debug, PartialEq, Eq, Default, Hash)]
pub struct FS {
    pub root: BTreeMap<String, FSNode>
}

#[derive(Debug, PartialEq, Eq, Default, Hash)]
pub struct FSNode {
    pub is_dir: bool,
    pub children: Vec<String>,
}

impl FS {
    pub fn insert_files(&mut self, files: Vec<String>) -> Vec<String> {
        let mut existed_files = vec![];
        for file in files {
            if !self.insert(&file, false) {
                existed_files.push(file);
            }
        }
        existed_files
    }

    fn insert(&mut self, file: &str, is_dir: bool) -> bool {
        let path = Path::new(file);
        if let Some(_) = self.root.get(file) {
            false
        } else if let Some(parent) = path.parent() {
            let parent = parent.to_str().unwrap();
            self.insert(parent, true);
            if let Some(parent_node) = self.root.get_mut(parent) {
                parent_node.children.push(path.file_name().unwrap().to_str().unwrap().to_string());
            }
            self.root.insert(file.into(), FSNode { is_dir: is_dir, children: vec![] });
            true
        } else { // root
            true
        }
    }
}
