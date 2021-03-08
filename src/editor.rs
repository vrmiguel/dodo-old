use std::path::PathBuf;

use colored::Colorize;
/// A simple wrapper over a Rustyline editor
use rustyline::{self, error::ReadlineError};

static HISTORY_FILE_NAME: &'static str = "ron_history";

pub struct Editor {
    inner: rustyline::Editor<()>,
}

impl Editor {
    /// Returns a new rustyline::Editor with history loaded in (if it exists)
    pub fn new(config_path: &PathBuf) -> Self {
        let mut inner = rustyline::Editor::<()>::new();
        // let hinter = EditorHinter {
        //     hints: match mode {
        //         ManagerMode::Tutorial   => tutorial_hints(),
        //         ManagerMode::Playground => playground_hints()
        //     }
        // };
        // editor.set_helper(Some(hinter));
        if inner
            .load_history(&config_path.join(HISTORY_FILE_NAME))
            .is_err()
        {}

        Self { inner }
    }

    pub fn read_line(&mut self, prompt: &str) -> Result<String, ReadlineError> {
        match self.inner.readline(prompt) {
            Ok(line) => {
                self.inner.add_history_entry(line.as_str());
                Ok(line)
            }
            err => err,
        }
    }

    pub fn save_history(&mut self, path: &PathBuf) {
        if let Err(err) = self.inner.save_history(&path.join(HISTORY_FILE_NAME)) {
            eprintln!("{}: problem saving history: {:?}", "warning".yellow(), err);
        }
    }

    pub fn show_error(err: rustyline::error::ReadlineError) {
        match err {
            ReadlineError::Interrupted => {
                println!("SIGINT received. Exiting.");
            }
            ReadlineError::Eof => {
                println!("EOF received. Exiting.");
            }
            err => {
                eprintln!("Error: {:?}", err);
            }
        }
    }
}
