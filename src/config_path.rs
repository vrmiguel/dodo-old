use std::{path::PathBuf, fs};
use directories::ProjectDirs;
use colored::Colorize;
use crate::errors::{self, Error};

/// get_config_path gets the platform-specific configuration folder path, creating it if it doesn't already exist.
pub fn get_config_path() -> Result<PathBuf, errors::Error> {
    let proj_dirs = ProjectDirs::from("com", "DodoOrg", "Dodo App");

    if proj_dirs.is_none() {
        return Err(Error::NoValidHomeDirFound);
    }

    let proj_dirs = proj_dirs.unwrap();
    let config_dir = proj_dirs.config_dir();

    if !config_dir.exists() {
        let folder_creation= fs::create_dir_all(config_dir);
        if folder_creation.is_err() {
            return Err(Error::CouldNotCreateFolder(PathBuf::from(config_dir)));
        }
        println!("{}: created folder {:?}", "info".yellow(), config_dir);
    }
    
    Ok(PathBuf::from(config_dir))
}