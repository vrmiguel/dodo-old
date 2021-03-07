use std::{fmt, io, path::PathBuf};

use ron;
pub enum Error {
    NoValidHomeDirFound,
    CouldNotCreateFolder(PathBuf),
    FileSystemError(io::Error),
    RonError(ron::error::ErrorCode),
    IoError(std::io::Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NoValidHomeDirFound => write!(
                f,
                "No valid home directory path could be retrieved from the operating system"
            ),
            Error::CouldNotCreateFolder(path_buf) => {
                write!(f, "Could not create path {:#?}", path_buf)
            },
            Error::FileSystemError(io_err) => {
                write!(f, "Could not load or save the save file: {:?}", io_err)
            },
            Error::RonError(ron_error_code) => {
                write!(f, "There's been a problem (de)serializing: {:?}", ron_error_code)
            }
            Error::IoError(io_error) => {
                write!(f, "There's been an IO problem: {}", io_error)
            }
        }
    }
}

impl From<ron::error::Error> for Error {
    fn from(err: ron::error::Error) -> Self {
        Self::RonError(err.code)
    }
}


impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}