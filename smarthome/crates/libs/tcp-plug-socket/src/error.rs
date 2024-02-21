use std::io;

use thiserror::{self, Error};

use crate::{Command, OddResponse};

#[derive(Debug, Error)]
pub enum Error {
    #[error("wrong response {resp} for command {cmd}")]
    WrongResponse { cmd: Command, resp: OddResponse },
    #[error("IO error")]
    IOError(#[from] io::Error),
}
