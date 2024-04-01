#[derive(Debug, Default, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Storage {
    pub storage_ip: String,
    pub client_port: u16,
    pub command_port: u16,
}
