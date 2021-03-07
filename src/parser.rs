use colored::Colorize;

/// Simplistic 'parser' for REPL arguments


#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    /// Flip a task from not done to done or vice-versa
    FlipTask(u16, u16),
    /// Adds a new task to a group;
    /// Arguments are (task description, group number)
    AddTask(String, u16),
    /// Adds a new group to the group list
    /// Argument is the new group's name
    AddGroup(String),
    /// Removes a task from a group.
    /// Arguments are (number of the task, number of the group)
    RemoveTask(u16, u16),
    /// Removes a group 
    /// Argument is the number of the group to be removed
    RemoveGroup(u16),
    NoOp
}

fn print_help() -> Command {
    println!("add help here");

    Command::NoOp
}

fn parse_task_addition(words: &Vec<&str>) -> Command {
    // add 2 "hahaha"
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


fn print_task_flip_help() -> Command {
    println!("{}: wrong format for argument to `{}`", "error".red(), "done".green());
    println!("Argument should follow the format `T.G`, where T represents the task number and G the group number.");
    Command::NoOp
}

fn parse_task_flip(words: &Vec<&str>) -> Command {
    // done 2.3
    if words.len() != 2 {
        println!("{}: wrong arguments to `{}`", "error".red(), "done".green());
        println!("Example usage: 'done 2.3', in order to mark the second task of the third group as done");
    }

    // let test = "23"
    // let ok = test.parse::<u16>();

    // Vec<Result<u16, std::num::ParseIntError>>
    let parts  = words[1]
        .split(".")
        .filter(|x| !x.is_empty())
        .map(|word| word.parse::<u16>());
        
    if parts.clone().any(|x| x.is_err()) {
        return print_task_flip_help();
    }

    let parts: Vec<u16> = parts.map(|x| x.unwrap()).collect();

    if parts.len() != 2 {
        return print_task_flip_help();
    }

    Command::FlipTask(parts[0], parts[1])
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
        // word if word.starts_with("remove") => {

        // },
        word if word.starts_with("group") => {
            parse_group_addition(&words)
        },
        word if word.starts_with("done") => {
            parse_task_flip(&words)
        }
        word => {
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
}