use std::{convert::TryFrom, fs::{self, File}, io::{BufWriter, Write}, path::PathBuf};

use ron;

use crate::errors::{self, Error};
use crate::task::TaskGroup;
use crate::config_path;

pub struct TodoList {
    pub task_groups: Vec<TaskGroup>,
    pub config_path: PathBuf
}

impl TryFrom<PathBuf> for TodoList {
    type Error = errors::Error;

    fn try_from(config_path: PathBuf) -> Result<Self, Self::Error> {
        let save_file_path = config_path.join("dodo.ron");
        let save_file = fs::read_to_string(save_file_path)?;
        let task_groups: Vec<TaskGroup> = ron::de::from_str(&save_file)?; 
        
        Ok(
            Self {
                task_groups,
                config_path
            }
        )
    }
}

impl TodoList {

    // pub fn new() -> Result<Self, Error> {
    //     let cfg_path = config_path::get_config_path();
    // }

    pub fn save_to_file(&self) -> Result<(), Error> {
        let serialized_data = ron::ser::to_string(&self.task_groups)?;
        let save_file = File::create(self.config_path.join("dodo.ron"))?;
        let mut writer = BufWriter::new(save_file);
        writer.write_all(serialized_data.as_bytes())?;
        Ok(())
    }
}