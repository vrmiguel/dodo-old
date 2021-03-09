use std::convert::TryFrom;

use clap::{self, Arg};

use crate::command::Command;
use crate::errors;

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
            .use_delimiter(false)
            .value_name("G")
            .help("Marks a task as done. Ex.: `dodo -d 3.2` marks the third task of the second group as done."),
        )
        .get_matches()
}

impl TryFrom<clap::ArgMatches<'static>> for Command {
    type Error = errors::Error;

    fn try_from(matches: clap::ArgMatches<'static>) -> Result<Self, Self::Error> {
        todo!()
    }
}