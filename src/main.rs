use std::{
    convert::TryFrom,
    env
};

use colored::Colorize;

mod cli;
mod command;
mod config_path;
mod editor;
mod errors;
mod macros;
mod parser;
mod repl;
mod task;
mod todolist;

fn main() -> Result<(), errors::Error> {
    
    let cfg_path = unwrap_or_return!(config_path::get_config_path());
    let list = todolist::TodoList::try_from(cfg_path.clone())?;
    if env::args().len() == 1 {
        print!("{}", list);
        return Ok(());
    }
    

    // let cfg_path = unwrap_or_return!(config_path::get_config_path());
    // let _matches = cli::get_matches();

    // let mut repl = repl::REPL::new(list)?;
    // repl.start()?;

    // let todo_list = todolist::TodoList::try_from(cfg_path)?;

    // println!("{}", todo_list);

    Ok(())
}
