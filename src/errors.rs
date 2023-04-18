use std::{error, fmt::Display, path::Path};

#[derive(Debug)]
// TODO: Move all error types here as this will be used when displaying errors in ui
pub enum AppError {
    WrongData(String),
    JsonError(json::Error),
    FileRead(String),
    FileStructure(Box<Path>)
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::WrongData(x) => write!(f, "Wrong data: {}", x),
            AppError::JsonError(x) => x.fmt(f),
            AppError::FileRead(x) => write!(f, "File Read: {}", x),
            AppError::FileStructure(x) => write!(f, "File structure: {}", x.display())
        }
    }
}

impl error::Error for AppError {}