use std::{collections::HashSet, path::PathBuf};

use colored::Colorize;

use rustyline::hint::{Hint, Hinter};
use rustyline::Context;
use rustyline::{self, error::ReadlineError};
use rustyline_derive::{Completer, Helper, Highlighter, Validator};

static HISTORY_FILE_NAME: &'static str = "ron_history";

impl Hint for CommandHint {
    fn display(&self) -> &str {
        &self.display
    }

    fn completion(&self) -> Option<&str> {
        if self.complete_up_to > 0 {
            Some(&self.display)
        } else {
            None
        }
    }
}

fn editor_hints() -> HashSet<CommandHint> {
    let mut set = HashSet::new();
    set.insert(CommandHint::new("add", "a"));
    set.insert(CommandHint::new("group", "g"));
    // set.insert(CommandHint::new("remove task",  "remove t"));
    set.insert(CommandHint::new("remove", "r"));
    set.insert(CommandHint::new("help", "h"));
    set
}

#[derive(Hash, Debug, PartialEq, Eq)]
struct CommandHint {
    display: String,
    complete_up_to: usize,
}

impl Hinter for EditorHinter {
    type Hint = CommandHint;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<CommandHint> {
        if pos < line.len() {
            return None;
        }

        self.hints
            .iter()
            .filter_map(|hint| {
                if pos > 0 && hint.display.starts_with(&line[..pos]) {
                    Some(hint.suffix(pos))
                } else {
                    None
                }
            })
            .next()
    }
}

#[derive(Completer, Helper, Validator, Highlighter)]
struct EditorHinter {
    hints: HashSet<CommandHint>,
}

impl CommandHint {
    fn new(text: &str, complete_up_to: &str) -> CommandHint {
        assert!(text.starts_with(complete_up_to));
        CommandHint {
            display: text.into(),
            complete_up_to: complete_up_to.len(),
        }
    }

    fn suffix(&self, strip_chars: usize) -> CommandHint {
        CommandHint {
            display: self.display[strip_chars..].to_owned(),
            complete_up_to: self.complete_up_to.saturating_sub(strip_chars),
        }
    }
}

/// A simple wrapper over a Rustyline editor
pub struct Editor {
    inner: rustyline::Editor<EditorHinter>,
}

impl Editor {
    /// Returns a new rustyline::Editor with history loaded in (if it exists)
    pub fn new(config_path: &PathBuf) -> Self {
        let mut inner = rustyline::Editor::<EditorHinter>::new();
        let hinter = EditorHinter {
            hints: editor_hints(),
        };
        inner.set_helper(Some(hinter));
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
