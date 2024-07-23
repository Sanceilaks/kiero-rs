use std::error::Error;


#[derive(Debug)]
pub enum ErrorKind {
    Unknown,
    NotSupported,
    ModuleNotFound(String),
    AlreadyInitialized,
    NotInitialized,
    IoError(Box<std::io::Error>),
}

#[derive(Debug)]
pub struct KieroError {
    pub kind: ErrorKind,
    pub message: String
}

impl KieroError {
    pub fn new(kind: ErrorKind, message: String) -> KieroError {
        KieroError {
            kind,
            message
        }
    }
}

impl std::fmt::Display for KieroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl Error for KieroError {}