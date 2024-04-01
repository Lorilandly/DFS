use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExceptionReturn {
    pub exception_type: String,
    pub exception_info: String,
}

impl ExceptionReturn {
    pub fn new(exception_type: &str, exception_info: &str) -> Self {
        Self {
            exception_type: exception_type.into(),
            exception_info: exception_info.into(),
        }
    }
}
