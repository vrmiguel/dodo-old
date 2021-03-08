/// Simplistic 'parser' for REPL arguments

use colored::Colorize;

use crate::command::Command;

fn print_help() -> Command {
    println!("TODO: add help");

    Command::NoOp
}

/// Given a string in the form "x.y", returns Some(x, y) or None
fn get_tuple<H>(word: &str, help_fn: H) -> Option<(u16, u16)> where 
    H: Fn() -> () {
    let parts  = word
        .split(".")
        .filter(|x| !x.is_empty())
        .map(|word| word.parse::<u16>());


    let show_help_and_exit = || {
        help_fn();
        None
    };

    if parts.clone().any(|x| x.is_err()) {
        return show_help_and_exit();
    }

    let parts: Vec<u16> = parts.map(|x| x.unwrap()).collect();

    if parts.len() != 2 {
        return show_help_and_exit();
    }

    Some((parts[0], parts[1]))
}


fn parse_task_flip(words: &Vec<&str>) -> Command {
    // Example: done 2.3
    if words.len() != 2 {
        println!("{}: wrong arguments to `{}`", "error".red(), "done".green());
        println!("Example usage: 'done 2.3', in order to mark the second task of the third group as done");
    }

    let task_flip_help = || {
        println!("{}: wrong format for argument to `{}`", "error".red(), "done".green());
        println!("Argument should follow the format `T.G`, where T represents the task number and G the group number.");
    };

    match get_tuple(words[1], task_flip_help) {
        Some((task_number, group_number)) => Command::FlipTask(task_number, group_number),
        None => Command::NoOp,
    }
}

fn parse_task_addition(words: &Vec<&str>) -> Command {
    // Example: add 2 "hahaha"
    if words.len() < 3 {
        println!("{}: missing arguments to `{}`", "error".red(), "add".green());
        println!("Example usage: add 2 \"Mop the floors\", in order to add \"Mop the floors\" to the group number 2.");
        return Command::NoOp;
    }
    
    let group_no = match words[1].parse::<u16>() {
        Ok(no) => no,
        Err(err) => {
            println!("{}: {}", "error".red(), err);
            return Command::NoOp;
        }
    };

    let task_description = words[2..].join(" ");

    Command::AddTask(task_description, group_no)
}

fn parse_group_addition(words: &Vec<&str>) -> Command {
    // Example: "group University"

    if words.len() < 2 {
        println!("{}: missing arguments to `{}`", "error".red(), "group".green());
        println!("Example usage: 'group University', in order to add a group named University");
        return Command::NoOp;
    }

    let group_name = words[1..].join(" ");

    Command::AddGroup(group_name)
}


fn parse_removal(words: &Vec<&str>) -> Command {
    // Examples:
    //     remove task 3.2
    //     remove group 2
    if words.len() != 3 {
        println!("{}: wrong arguments to `{}`", "error".red(), "remove".green());
    }

    let print_usage = || {};

    let get_group_to_remove = |word: &str| {
        match word.parse::<u16>() {
            Ok(num) => Some(num),
            Err(_) => {
                print_usage();
                None
            }
        }
    };

    match words[1] {
        "group" => {
            match get_group_to_remove(words[2]) {
                Some(group_no) => Command::RemoveGroup(group_no),
                None => Command::NoOp
            }
        },
        "task" => {
            match get_tuple(words[2], print_usage) {
                Some((task_no, group_no)) => Command::RemoveTask(task_no, group_no),
                None => Command::NoOp
            }
        },
        other => {
            println!("{}: expected {} or {}, found {}", "error".red(), "task".green(), "group".green(), other.red());
            Command::NoOp       
        }
    }
}

pub fn parse(line: &str) -> Command {
    let words: Vec<&str> = line.split(' ').filter(|x| !x.is_empty()).collect();
    if words.is_empty() {
        return Command::NoOp;
    }
    let first_word = words[0];
    match first_word {
        word if word.starts_with("help") => {
            print_help()
        }, 
        word if word.starts_with("add") => {
            parse_task_addition(&words)
        },
        word if word.starts_with("remove") => {
            parse_removal(&words)
        },
        word if word.starts_with("group") => {
            parse_group_addition(&words)
        },
        word if word.starts_with("done") => {
            parse_task_flip(&words)
        }
        word => {
            println!("{}: \"{}\" is not a recognized command. \nType in `help` to get additional help.", "error".red(), word);
            Command::NoOp
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn task_addition_1() {
        let line = "add 3 \"Mop the floor\"";

        assert_eq!(
            Command::AddTask(
                "\"Mop the floor\"".into(),
                3,
            ),
            parse(line)
        );
    }

    #[test]
    fn task_addition_2() {
        let line = "add 1 \"Read a book\"";

        assert_eq!(
            Command::AddTask(
                "\"Read a book\"".into(),
                1,
            ),
            parse(line)
        );
    }

    #[test]
    fn task_addition_3() {
        let line = "add -1 \"Read a book\"";

        assert_eq!(
            // Should fail upon group number parsing and return NoOp
            Command::NoOp,
            parse(line)
        );
    }

    #[test]
    fn group_addition() {

        let line = "group University";
        
        assert_eq!(
            parse(line),
            Command::AddGroup(
                "University".into(),
            )
        )
    }

    #[test]
    fn task_flip() {

        assert_eq!(
            parse("done 2.3"),
            Command::FlipTask(
                2,
                3,
            )
        )
    }

    #[test]
    fn task_removal() {
        assert_eq!(
            parse("remove task 2.3"),
            Command::RemoveTask(2, 3)
        );
    }

    #[test]
    fn group_removal() {
        assert_eq!(
            parse("remove group 2"),
            Command::RemoveGroup(2)
        );
    }
}