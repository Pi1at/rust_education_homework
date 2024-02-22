use std::io;

use thiserror::{self, Error};

use crate::{Command, OddResponse};

#[derive(Debug, Error)]
pub enum Error {
    /// Error indicating a wrong response was received for a given command.
    #[error("wrong response {resp} for command {cmd}")]
    WrongResponse { cmd: Command, resp: OddResponse },
    /// Wrapper around the standard IO error to integrate with custom error handling.
    #[error("IO error")]
    IOError(#[from] io::Error),
}
