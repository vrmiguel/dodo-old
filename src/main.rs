use std::convert::TryFrom;

use colored::Colorize;

mod cli;
mod config_path;
mod errors;
mod macros;
mod todolist;
mod task;
mod repl;
mod editor;
mod parser;
mod command;

use task::{Task, TaskGroup};

fn main() -> Result<(), errors::Error> {
    let cfg_path = unwrap_or_return!(config_path::get_config_path());
    let matches = cli::get_matches();
    let list = todolist::TodoList::try_from(cfg_path.clone())?;
    
    let groups = vec![TaskGroup {
        name: "Uni".into(),
        tasks: vec![
            Task {
                description: "Study for the Physics test".into(),
                is_done: true,
            },
            Task {
                description: "Study Monads".into(),
                is_done: true,
            },
            Task {
                description: "Finish the Compilers project".into(),
                is_done: false,
            },
        ],
    },
    TaskGroup {
        name: "College".into(),
        tasks: vec![
            Task {
                description: "Study for the Maths test".into(),
                is_done: true,
            },
            Task {
                description: "Study the life of Archimedes".into(),
                is_done: true,
            },
        ],
        }];
    
    let mut repl = repl::REPL::new(list)?;
    repl.start()?;
    

    let todo_list = todolist::TodoList::try_from(cfg_path)?;


    println!("{}", todo_list);

    // dbg!(save_file::load_save_file(&cfg_path));

    // let _ = save_file::create_save_file(&groups, &cfg_path);

    Ok(())
}
