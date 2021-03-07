use std::path::{Path, PathBuf};

/// A simple wrapper over a Rustyline editor

use rustyline::{self, error::ReadlineError};
use colored::Colorize;

pub struct Editor {
    inner: rustyline::Editor<()>,
}

impl Editor {
    /// Returns a new rustyline::Editor with history loaded in (if it exists)
    pub fn new() -> Self {
        let mut inner = rustyline::Editor::<()>::new();
        // let hinter = EditorHinter {
        //     hints: match mode {
        //         ManagerMode::Tutorial   => tutorial_hints(),
        //         ManagerMode::Playground => playground_hints()
        //     }
        // };
        // editor.set_helper(Some(hinter));
        if inner.load_history("history.txt").is_err() {}

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
        if let Err(err) = self.inner.save_history(&path.join("history.txt")) {
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
