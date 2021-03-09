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
    let mut list = todolist::TodoList::try_from(cfg_path)?;
    if env::args().len() == 1 {
        print!("{}", list);
        return Ok(());
    }
    let matches = cli::get_matches();
    let args = cli::CommandLineArguments::try_from(matches)?;

    for command in args.commands {
        list.evaluate(command);
    }

    if args.should_start_repl {
        let mut repl = repl::REPL::new(list)?;
        repl.start_loop()?;
    } else {
        println!("{}", list);
    }

    Ok(())
}
