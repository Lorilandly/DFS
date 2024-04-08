//! This module contains the struct for the exception return.
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

/// Represents the response payload for an exception.
pub struct ExceptionReturn {
    /// The type of the exception.
    pub exception_type: String,
    /// The information of the exception.
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
