use std::collections::{BTreeMap, HashSet};

pub struct Dfs {
    pub storage: HashSet<Storage>,
    pub fs: BTreeMap<String, Vec<FSNode>>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Storage {
    pub storage_ip: String,
    pub client_port: u16,
    pub command_port: u16,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FSNode {
    pub is_dir: bool,
    pub children: Vec<String>,
}
