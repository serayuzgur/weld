#[derive(Clone)]
#[derive(Debug)]
pub enum Errors {
    NotFound(String),
    Duplicate(String),
}