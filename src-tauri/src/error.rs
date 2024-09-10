use std::io;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BackendError {
    pub(crate) kind: String,
    pub(crate) message: String,
}

impl From<io::Error> for BackendError {
    fn from(value: io::Error) -> Self {
        Self {
            kind: String::from("std::io"),
            message: value.to_string(),
        }
    }
}

impl From<sea_orm::DbErr> for BackendError {
    fn from(value: sea_orm::DbErr) -> Self {
        Self {
            kind: String::from("sea_orm::DbErr"),
            message: value.to_string(),
        }
    }
}
