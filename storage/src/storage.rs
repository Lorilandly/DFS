use crate::handlers::exception_return::ExceptionReturn;
use axum::response::IntoResponse;
use std::fs;
use std::os::unix::fs::FileExt;
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

    fn get_full_path(&self, path: &Path) -> Result<PathBuf, impl IntoResponse> {
        match path.strip_prefix("/") {
            Ok(p) => Ok(Path::new(&self.root_storage_dir).join(p)),
            Err(_) => Err(axum::Json(ExceptionReturn::new(
                "IllegalArgumentException",
                "path must start with /",
            ))
            .into_response()),
        }
    }

    pub fn get_file_size(&self, path: &Path) -> Result<u64, impl IntoResponse> {
        match self.get_full_path(path) {
            Ok(p) => match fs::metadata(&p) {
                Ok(meta) => {
                    if meta.is_dir() {
                        Err(axum::Json(ExceptionReturn::new(
                            "FileNotFoundException",
                            "path must be a file",
                        ))
                        .into_response())
                    } else {
                        Ok(meta.len())
                    }
                }
                Err(_) => Err(axum::Json(ExceptionReturn::new(
                    "FileNotFoundException",
                    "the parent directory does not exist.",
                ))
                .into_response()),
            },
            Err(e) => Err(e.into_response()),
        }
    }

    pub fn find_file(&self, path: &Path) -> Result<fs::File, impl IntoResponse> {
        match self.get_full_path(path) {
            Ok(full_path) => match fs::metadata(&full_path) {
                Ok(meta) => {
                    if meta.is_dir() {
                        Err(axum::Json(ExceptionReturn::new(
                            "FileNotFoundException",
                            "path must be a file",
                        ))
                        .into_response())
                    } else {
                        match fs::OpenOptions::new()
                            .write(true)
                            .read(true)
                            .open(&full_path)
                        {
                            Ok(file) => Ok(file),
                            Err(_) => Err(axum::Json(ExceptionReturn::new(
                                "FileNotFoundException",
                                "the file does not exist.",
                            ))
                            .into_response()),
                        }
                    }
                }
                Err(_) => Err(axum::Json(ExceptionReturn::new(
                    "FileNotFoundException",
                    "the file does not exist.",
                ))
                .into_response()),
            },
            Err(e) => Err(e.into_response()),
        }
    }

    pub fn read(
        &self,
        path: &Path,
        offset: u64,
        length: u64,
    ) -> Result<Vec<u8>, impl IntoResponse> {
        match self.find_file(path) {
            Ok(file) => {
                let mut buffer = vec![0; length as usize];
                match file.read_at(&mut buffer, offset) {
                    Ok(read_size) => {
                        if read_size < length as usize {
                            Err(axum::Json(ExceptionReturn::new(
                                "IndexOutOfBoundsException",
                                "Read past end of file.",
                            ))
                            .into_response())
                        } else {
                            Ok(buffer)
                        }
                    }
                    Err(_) => Err(axum::Json(ExceptionReturn::new(
                        "FileNotFoundException",
                        "error reading file",
                    ))
                    .into_response()),
                }
            }
            Err(e) => Err(e.into_response()),
        }
    }

    pub fn write(&self, path: &Path, offset: u64, data: Vec<u8>) -> Result<(), impl IntoResponse> {
        match self.find_file(path) {
            Ok(file) => match file.write_at(&data, offset) {
                Ok(_) => Ok(()),
                Err(e) => {
                    eprintln!("error writing file: {:?} {:?}", file, e);
                    Err(axum::Json(ExceptionReturn::new(
                        "FileNotFoundException",
                        "error writing file",
                    ))
                    .into_response())
                }
            },
            Err(e) => Err(e.into_response()),
        }
    }

    pub fn create_file(&self, path: &Path) -> Result<bool, impl IntoResponse> {
        if path == Path::new("/") {
            return Err(axum::Json(ExceptionReturn::new(
                "IllegalArgumentException",
                "path cannot be empty.",
            ))
            .into_response());
        }
        match self.get_full_path(path) {
            Ok(full_path) => match fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&full_path)
            {
                Ok(_) => Ok(true),
                Err(_) => Err(axum::Json(ExceptionReturn::new(
                    "FileNotFoundException",
                    "the file already exists.",
                ))
                .into_response()),
            },
            Err(e) => Err(e.into_response()),
        }
    }
}
