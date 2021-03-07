use std::{fs::File, path::PathBuf};

use crate::errors::Error;

pub fn load_save_file(cfg_path: &PathBuf) -> Result<File, Error> {
    let save_file_path = cfg_path.join("dodo.ron");
    match File::open(save_file_path) {
        Ok(file) => Ok(file),
        Err(err) => Err(Error::CouldNotLoadSaveFile(err)),
    }
}

pub fn create_save_file() {}
