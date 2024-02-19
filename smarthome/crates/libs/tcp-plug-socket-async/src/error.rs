use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct WrongResponse {
    message: String,
}

impl WrongResponse {
    pub const fn new(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for WrongResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WrongResponse: {}", self.message)
    }
}

impl error::Error for WrongResponse {}
