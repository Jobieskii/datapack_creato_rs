use std::{error, fmt::Display};

#[derive(Debug)]
// TODO: Move all error types here as this will be used when displaying errors in ui
pub enum AppError {
    WrongData(String),
    JsonError(json::Error)
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::WrongData(x) => write!(f, "Wrong data: {}", x),
            AppError::JsonError(x) => x.fmt(f),
        }
    }
}

impl error::Error for AppError {}