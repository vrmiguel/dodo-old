use std::path::PathBuf;

use crate::{editor::{self, Editor}, errors, save_file, task::{Task, TaskGroup}};


/// dodo's Read-Eval-Print Loop
pub struct REPL {
    task_groups: Vec<TaskGroup>,
    config_path: PathBuf,
    editor: Editor
}

impl REPL {
    pub fn new(task_groups: Vec<TaskGroup>, config_path: PathBuf) -> Result<Self, errors::Error> {

        let task_groups = save_file::load_save_file(&config_path)?;

        Ok(Self {
            task_groups,
            config_path,
            editor: Editor::new()
        })
    }

    pub fn parse(&mut self, line: &str) {

    }

    /// Starts the loop until an exit signal is given
    pub fn start(&mut self) -> Result<(), errors::Error> {
        loop {
            match self.editor.read_line(">> ") {
                Ok(line) => self.parse(line.as_str()),
                Err(err) => {
                    // Prints some additional info depending on which error we're getting
                    Editor::show_error(err);
                    self.editor.save_history(&self.config_path);
                    return Ok(());
                }
            }
        }
    }
}