use std::fs;
use std::path::Path;
use std::path::PathBuf;
pub struct Storage {
    root_storage_dir: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RegisterRequest {
    pub storage_ip: String,
    pub client_port: u16,
    pub command_port: u16,
    pub files: Vec<PathBuf>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RegisterResponse {
    pub files: Vec<PathBuf>,
}

impl Storage {
    pub fn new(root: &Path) -> Self {
        Self {
            root_storage_dir: root.to_str().unwrap().to_string(),
        }
    }

    pub async fn initialize_storage(
        &self,
        client_port: u16,
        command_port: u16,
        registration_port: u16,
        root_storage_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();

        let files = self
            .get_all_files_recursive(root_storage_dir)
            .expect("Error when getting files in storage directory");
        println!("files: {:?}", files);
        let res = client
            .post(format!("http://localhost:{}/register", registration_port))
            .json(&RegisterRequest {
                storage_ip: "localhost".to_string(),
                client_port,
                command_port,
                files,
            })
            .send()?;

        let response = res.json::<RegisterResponse>()?;
        println!("response: {:?}", response);
        for file in response.files {
            let path_to_delete = root_storage_dir.join(file.strip_prefix("/")?);
            println!("deleting: {:?}", path_to_delete);
            fs::remove_file(&path_to_delete)?;
            if let Some(parent) = path_to_delete.parent() {
                self.remove_dir_recursive(parent)?;
            }
        }
        Ok(())
    }

    pub fn remove_dir_recursive(&self, path: &Path) -> Result<(), std::io::Error> {
        if self.is_dir_empty(path)? {
            fs::remove_dir(path)?;
            if let Some(parent) = path.parent() {
                self.remove_dir_recursive(parent)?;
            }
        }
        Ok(())
    }

    pub fn is_dir_empty(&self, path: &Path) -> Result<bool, std::io::Error> {
        Ok(fs::read_dir(path)?.next().is_none())
    }

    pub fn get_all_files_recursive(&self, path: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
        let mut files = vec![];
        for entry in fs::read_dir(path)? {
            let path = entry?.path();
            if path.is_dir() {
                files.append(&mut self.get_all_files_recursive(&path)?)
            } else {
                files.push(Path::new("/").join(path.strip_prefix(&self.root_storage_dir).unwrap()))
            }
        }
        Ok(files)
    }
}
