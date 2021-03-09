use colored::Colorize;

use crate::{
    editor::Editor,
    errors, parser,
    todolist::TodoList,
};

/// dodo's Read-Eval-Print Loop
pub struct REPL {
    todo_list: TodoList,
    editor: Editor,
}

impl REPL {
    pub fn new(todo_list: TodoList) -> Result<Self, errors::Error> {
        Ok(Self {
            editor: Editor::new(&todo_list.config_path),
            todo_list,
        })
    }

    /// Starts the loop until an exit signal is given
    pub fn start(&mut self) -> Result<(), errors::Error> {
        loop {
            match self.editor.read_line(">> ") {
                Ok(line) => {
                    let command = parser::parse(line.as_str());
                    self.todo_list.evaluate(command);
                }
                Err(err) => {
                    // Prints some additional info depending on which error we're getting
                    Editor::show_error(err);
                    self.editor.save_history(&self.todo_list.config_path);
                    if let Err(err) = self.todo_list.save_to_file() {
                        println!("{}: {}", "error".red(), err);
                    }
                    return Ok(());
                }
            }
        }
    }
}
