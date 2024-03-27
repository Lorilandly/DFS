use std::collections::BTreeMap;

pub struct Storage {
    files: Vec<String>,
}

impl Default for Storage {
    fn default() -> Self {
        let files = vec![];
        Self { files }
    }
}

impl Storage {
    pub fn insert(&mut self, file: String) {
        self.files.push(file);
    }
}
