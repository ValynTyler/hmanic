use std::env;
use std::io;

#[derive(Debug)]
pub enum Error {
    EnvError(env::VarError),
    IoError(io::Error),
}

impl From<env::VarError> for Error {
    fn from(err: env::VarError) -> Error {
        Error::EnvError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}
