use std::convert::TryFrom;

use clap::{self, Arg};

use crate::command::Command;
use crate::errors;
use crate::parser;

pub fn get_matches() -> clap::ArgMatches<'static> {
    clap::App::new("dodo")
        .version("0.1.0")
        .about("A to-do list application for your terminal")
        .help_message("Displays this message and exits")
        .settings(&[
            clap::AppSettings::ColoredHelp,
        ])
        .arg(
            Arg::with_name("edit")
                .required(false)
                .long("edit")
                .short("e")
                // .alias("repl")
                .takes_value(false)
                .help("Starts a REPL for interactive editing of the list "),
        )
        .arg(
            Arg::with_name("done")
            .required(false)
            .long("done")
            .short("d")
            .takes_value(true)
            .value_name("T.G")
            .help("Marks a task as done. Ex.: `dodo -d 3.2` marks the third task of the second group as done."),
        )
        .arg(
            Arg::with_name("task")
            .required(false)
            .long("task")
            .short("t")
            .takes_value(true)
            .multiple(true)
            .value_name("G")
            .help("Marks a task as done. Ex.: `dodo -d 3.2` marks the third task of the second group as done."),
        )
        .get_matches()
}

pub struct CommandLineArguments {
    pub should_start_repl: bool,
    pub commands: Vec<Command>
}

impl TryFrom<clap::ArgMatches<'static>> for CommandLineArguments {
    type Error = errors::Error;

    fn try_from(matches: clap::ArgMatches<'static>) -> Result<Self, Self::Error> {
        let mut commands: Vec<Command> = vec![];
        
        let parse_options = |matches: &clap::ArgMatches<'static>, commands: & mut Vec<Command>, word | {
            if matches.is_present(word) {
                let mut values: Vec<&str> = vec![word];
                let mut args: Vec<&str> = matches.values_of(word).unwrap().collect();
                values.append(&mut args);
                // TODO: stop execution when a NoOp is found?
                commands.push(parser::parse(&values.join(" ")));
            }
        };

        let should_start_repl = matches.is_present("edit");

        let options = ["task", "group", "done"];

        for option in &options {
            parse_options(&matches, &mut commands, option);
        }

        Ok(
            Self {
                commands,
                should_start_repl
            }
        )
    }
}