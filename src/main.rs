use colored::Colorize;

mod cli;
mod config_path;
mod errors;
mod macros;
mod save_file;
mod task;

fn main() {
    let cfg_path = unwrap_or_return!(config_path::get_config_path());
    let matches = cli::get_matches();
    dbg!(&matches);
}
