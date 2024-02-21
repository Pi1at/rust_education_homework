use std::io;

use thiserror::{self, Error};

use crate::{Command, OddResponse};

#[derive(Debug, Error)]
pub enum Error {
    #[error("wrong response {1} for command {0}")]
    WrongResponse(Command, OddResponse),
    #[error("IO error")]
    IOError(#[from] io::Error),
}
