use colored::Colorize;

mod cli;
mod config_path;
mod errors;
mod macros;
mod save_file;
mod task;
mod repl;
mod editor;

use task::{Task, TaskGroup};

fn main() {
    let cfg_path = unwrap_or_return!(config_path::get_config_path());
    let matches = cli::get_matches();
    
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
    
    let repl = repl::REPL::new(groups);
    
}