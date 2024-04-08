use crate::handlers::{exception_return::ExceptionReturn, storage_read::StorageReadRequest};
use crate::requests::{get_file_size, read_file};
use axum::{http::StatusCode, Json};
use std::{
    fs,
    fs::File,
    io::Write,
    os::unix::fs::FileExt,
    path::{Path, PathBuf},
};

type Result<T> = std::result::Result<T, (StatusCode, Json<ExceptionReturn>)>;

/// Storage struct
pub struct Storage {
    /// the root directory of the storage server
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
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
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

    pub fn remove_dir_recursive(&self, path: &Path) -> std::result::Result<(), std::io::Error> {
        if self.is_dir_empty(path)? {
            fs::remove_dir(path)?;
            if let Some(parent) = path.parent() {
                self.remove_dir_recursive(parent)?;
            }
        }
        Ok(())
    }

    pub fn is_dir_empty(&self, path: &Path) -> std::result::Result<bool, std::io::Error> {
        Ok(fs::read_dir(path)?.next().is_none())
    }

    pub fn get_all_files_recursive(
        &self,
        path: &Path,
    ) -> std::result::Result<Vec<PathBuf>, std::io::Error> {
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

    fn get_full_path(&self, path: &Path) -> Result<PathBuf> {
        match path.strip_prefix("/") {
            Ok(p) => Ok(Path::new(&self.root_storage_dir).join(p)),
            Err(_) => Err((
                StatusCode::BAD_REQUEST,
                Json(ExceptionReturn::new(
                    "IllegalArgumentException",
                    "path must start with /",
                )),
            )),
        }
    }

    pub fn get_file_size(&self, path: &Path) -> Result<u64> {
        let full_path = self.get_full_path(path)?;
        match fs::metadata(&full_path) {
            Ok(meta) => {
                if meta.is_dir() {
                    Err((
                        StatusCode::BAD_REQUEST,
                        Json(ExceptionReturn::new(
                            "FileNotFoundException",
                            "path must be a file",
                        )),
                    ))
                } else {
                    Ok(meta.len())
                }
            }
            Err(_) => Err((
                StatusCode::BAD_REQUEST,
                axum::Json(ExceptionReturn::new(
                    "FileNotFoundException",
                    "the parent directory does not exist.",
                )),
            )),
        }
    }

    pub fn find_file(&self, path: &Path) -> Result<fs::File> {
        let full_path = self.get_full_path(path)?;
        match fs::metadata(&full_path) {
            Ok(meta) => {
                if meta.is_dir() {
                    Err((
                        StatusCode::BAD_REQUEST,
                        axum::Json(ExceptionReturn::new(
                            "FileNotFoundException",
                            "path must be a file",
                        )),
                    ))
                } else {
                    match fs::OpenOptions::new()
                        .write(true)
                        .read(true)
                        .open(&full_path)
                    {
                        Ok(file) => Ok(file),
                        Err(_) => Err((
                            StatusCode::BAD_REQUEST,
                            axum::Json(ExceptionReturn::new(
                                "FileNotFoundException",
                                "the file does not exist.",
                            )),
                        )),
                    }
                }
            }
            Err(_) => Err((
                StatusCode::BAD_REQUEST,
                axum::Json(ExceptionReturn::new(
                    "FileNotFoundException",
                    "the file does not exist.",
                )),
            )),
        }
    }

    pub fn read(&self, path: &Path, offset: u64, length: u64) -> Result<Vec<u8>> {
        let file = self.find_file(path)?;
        let mut buffer = vec![0; length as usize];
        match file.read_at(&mut buffer, offset) {
            Ok(read_size) => {
                if read_size < length as usize {
                    Err((
                        StatusCode::BAD_REQUEST,
                        axum::Json(ExceptionReturn::new(
                            "IndexOutOfBoundsException",
                            "Read past end of file.",
                        )),
                    ))
                } else {
                    Ok(buffer)
                }
            }
            Err(_) => Err((
                StatusCode::BAD_REQUEST,
                axum::Json(ExceptionReturn::new(
                    "FileNotFoundException",
                    "error reading file",
                )),
            )),
        }
    }

    pub fn write(&self, path: &Path, offset: i64, data: Vec<u8>) -> Result<()> {
        if offset < 0 {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ExceptionReturn::new(
                    "IndexOutOfBoundsException",
                    "Offset or length cannot be negative",
                )),
            ));
        }
        let file = self.find_file(path)?;
        match file.write_at(&data, offset as u64) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("error writing file: {:?} {:?}", file, e);
                Err((
                    StatusCode::BAD_REQUEST,
                    axum::Json(ExceptionReturn::new(
                        "FileNotFoundException",
                        "error writing file",
                    )),
                ))
            }
        }
    }

    pub fn create_file(&self, path: &Path) -> Result<bool> {
        if path == Path::new("/") {
            return Err((
                StatusCode::BAD_REQUEST,
                axum::Json(ExceptionReturn::new(
                    "IllegalArgumentException",
                    "path cannot be empty.",
                )),
            ));
        }
        let full_path = self.get_full_path(path)?;
        match fs::create_dir_all(full_path.parent().unwrap()) {
            Ok(_) => {
                match fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&full_path)
                {
                    Ok(_) => Ok(true),
                    Err(e) => Err((
                        StatusCode::BAD_REQUEST,
                        axum::Json(ExceptionReturn::new(
                            "IllegalArgumentException",
                            &e.to_string(),
                        )),
                    )),
                }
            }
            Err(e) => Err((
                StatusCode::BAD_REQUEST,
                axum::Json(ExceptionReturn::new(
                    "IllegalArgumentException",
                    &e.to_string(),
                )),
            )),
        }
    }

    pub fn delete_file(&self, path: &Path) -> Result<bool> {
        if path == Path::new("/") {
            return Err((
                StatusCode::BAD_REQUEST,
                axum::Json(ExceptionReturn::new(
                    "IllegalArgumentException",
                    "path cannot be root.",
                )),
            ));
        }
        let full_path = self.get_full_path(path)?;
        match fs::metadata(&full_path) {
            Ok(meta) => {
                if meta.is_dir() {
                    // delete directory
                    match fs::remove_dir_all(&full_path) {
                        Ok(_) => Ok(true),
                        Err(e) => Err((
                            StatusCode::BAD_REQUEST,
                            axum::Json(ExceptionReturn::new(
                                "FileNotFoundException",
                                &e.to_string(),
                            )),
                        )),
                    }
                } else {
                    match fs::remove_file(&full_path) {
                        Ok(_) => Ok(true),
                        Err(e) => Err((
                            StatusCode::BAD_REQUEST,
                            axum::Json(ExceptionReturn::new(
                                "FileNotFoundException",
                                &e.to_string(),
                            )),
                        )),
                    }
                }
            }
            Err(e) => Err((
                StatusCode::BAD_REQUEST,
                axum::Json(ExceptionReturn::new(
                    "FileNotFoundException",
                    &e.to_string(),
                )),
            )),
        }
    }

    pub async fn copy(&self, path: &Path, server_ip: &str, server_port: u16) -> Result<bool> {
        if !Storage::is_valid_path(path) {
            return Err((
                StatusCode::BAD_REQUEST,
                axum::Json(ExceptionReturn::new(
                    "IllegalArgumentException",
                    "path must be absolute",
                )),
            ));
        }
        // get file size from the server with given ip and port
        let res = get_file_size(
            path.to_path_buf(),
            server_ip.to_string(),
            server_port as i16,
        )
        .await;

        match res {
            Ok(res) => {
                let size = res.size;
                let offset = 0;
                let storage_read_request = StorageReadRequest {
                    path: path.to_path_buf(),
                    offset,
                    length: size as i64,
                };
                // read file from the server with given ip and port
                match read_file(
                    storage_read_request,
                    server_ip.to_string(),
                    server_port as i16,
                )
                .await
                {
                    Ok(res) => {
                        let data = base64::decode(&res.data).unwrap();
                        let full_path = self.get_full_path(path)?;

                        self.create_file_ignore_exist(path)?;
                        let mut f = std::fs::OpenOptions::new()
                            .write(true)
                            .open(full_path)
                            .unwrap();
                        f.write_all(&data).unwrap();

                        Ok(true)
                    }
                    Err(e) => Err((
                        StatusCode::BAD_REQUEST,
                        axum::Json(ExceptionReturn::new(
                            "FileNotFoundException",
                            &e.to_string(),
                        )),
                    )),
                }
            }
            Err(e) => Err((StatusCode::BAD_REQUEST, Json(e))),
        }
    }

    pub fn is_valid_path(path: &Path) -> bool {
        path.is_absolute()
    }

    pub fn create_file_ignore_exist(&self, path: &Path) -> Result<File> {
        let full_path = self.get_full_path(path)?;
        match fs::create_dir_all(full_path.parent().unwrap()) {
            Ok(_) => match fs::remove_file(&full_path) {
                Ok(_) => match fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&full_path)
                {
                    Ok(f) => Ok(f),
                    Err(e) => Err((
                        StatusCode::BAD_REQUEST,
                        axum::Json(ExceptionReturn::new(
                            "FileNotFoundException",
                            &e.to_string(),
                        )),
                    )),
                },
                Err(_) => match fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(&full_path)
                {
                    Ok(f) => Ok(f),
                    Err(e) => Err((
                        StatusCode::BAD_REQUEST,
                        axum::Json(ExceptionReturn::new(
                            "FileNotFoundException",
                            &e.to_string(),
                        )),
                    )),
                },
            },
            Err(e) => Err((
                StatusCode::BAD_REQUEST,
                axum::Json(ExceptionReturn::new(
                    "FileNotFoundException",
                    &e.to_string(),
                )),
            )),
        }
    }
}
