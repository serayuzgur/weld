//! # errors
// This module includes error enum and utility fuctions.
use slog::Logger;
use serde_json::Value;

/// Error types which can occur on database aperation.
#[derive(Clone)]
#[derive(Debug)]
pub enum Errors {
    /// Record not found
    NotFound(String),
    /// Record conflicts wit another record.
    Conflict(String),
}

/// Takes the error, logs it and wraps inside a Result
pub fn log_n_wrap(logger: &Logger, error: Errors) -> Result<Value, Errors> {
    error!(logger, "{:?}", error);
    return Err(error);
}