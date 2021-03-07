use std::{fs::File, io::{BufWriter, Write}, path::PathBuf};

use ron;

use crate::errors::Error;
use crate::task::TaskGroup;

pub fn load_save_file(cfg_path: &PathBuf) -> Result<File, Error> {
    let save_file_path = cfg_path.join("dodo.ron");
    Ok(File::open(save_file_path)?)
}

pub fn create_save_file(groups: &Vec<TaskGroup>, cfg_path: &PathBuf) -> Result<(), Error>{
    let serialized_data = ron::ser::to_string(groups)?;
    let save_file = File::create(cfg_path.join("dodo.ron"))?;
    let mut writer = BufWriter::new(save_file);
    writer.write_all(serialized_data.as_bytes())?;
    Ok(())
}
