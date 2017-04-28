use slog::Logger;
use serde_json::Value;

#[derive(Clone)]
#[derive(Debug)]
pub enum Errors {
    NotFound(String),
    Duplicate(String),
}

/// Takes the error, logs it and wraps inside a Result
pub fn log_n_wrap(logger: &Logger, error: Errors) -> Result<Value, Errors> {
    error!(logger, "{:?}", error);
    return Err(error);
}