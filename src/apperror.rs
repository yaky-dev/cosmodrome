use std::io;
use std::fmt;

#[derive(Debug)]
pub struct AppError {
    kind: String,    // type of the error
    message: String, // error message
}

impl AppError {
    pub fn new(message: &str) -> Self {
        AppError {
            kind: String::from("app"),
            message: String::from(message)
        }
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            kind: String::from("io"),
            message: error.to_string()
        }
    }
}

impl From<&str> for AppError {
    fn from(message: &str) -> Self {
        AppError {
            kind: String::from("app"),
            message: message.to_string()
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}) {}", self.kind, self.message)
    }
}