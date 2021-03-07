use std::{fmt, io, path::PathBuf};
pub enum Error {
    NoValidHomeDirFound,
    CouldNotCreateFolder(PathBuf),
    CouldNotLoadSaveFile(io::Error),
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
            }
            Error::CouldNotLoadSaveFile(io_err) => {
                write!(f, "Could not load save file: {:?}", io_err)
            }
        }
    }
}
