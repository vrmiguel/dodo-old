use std::{
    convert::TryFrom,
    fmt,
    fs::{self, File},
    io::{BufWriter, Write},
    path::PathBuf,
};

use colored::Colorize;
use ron;

use crate::command::Command;
use crate::config_path;
use crate::errors::{self, Error};
use crate::task::{Task, TaskGroup};

#[derive(Debug, PartialEq, Eq)]
pub struct TodoList {
    pub task_groups: Vec<TaskGroup>,
    pub config_path: PathBuf,
}

impl fmt::Display for TodoList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, group) in self.task_groups.iter().enumerate() {
            writeln!(f, "{}. {}", i + 1, group)?;
        }

        write!(f, "")
    }
}

impl TryFrom<PathBuf> for TodoList {
    type Error = errors::Error;

    fn try_from(config_path: PathBuf) -> Result<Self, Self::Error> {
        let save_file_path = config_path.join("dodo.ron");

        let task_groups: Vec<TaskGroup> = match save_file_path.exists() {
            true => {
                let file_contents = fs::read_to_string(save_file_path)?;
                ron::de::from_str(&file_contents)?
            }
            false => {
                println!(
                    "{}: could not find a `dodo.ron` in `{:#?}`. \nCreating a new save file.",
                    "warning".yellow(),
                    save_file_path
                );
                vec![]
            }
        };

        Ok(Self {
            task_groups,
            config_path,
        })
    }
}

impl TryFrom<Vec<TaskGroup>> for TodoList {
    type Error = errors::Error;

    fn try_from(task_groups: Vec<TaskGroup>) -> Result<Self, Self::Error> {
        let config_path = config_path::get_config_path()?;

        Ok(Self {
            task_groups,
            config_path,
        })
    }
}

impl TodoList {
    // pub fn new() -> Result<Self, Error> {
    //     let cfg_path = config_path::get_config_path();
    // }

    pub fn save_to_file(&self) -> Result<(), Error> {
        let serialized_data = ron::ser::to_string(&self.task_groups)?;
        let save_file = File::create(self.config_path.join("dodo.ron"))?;
        let mut writer = BufWriter::new(save_file);
        writer.write_all(serialized_data.as_bytes())?;
        Ok(())
    }

    fn add_task(&mut self, description: String, group_no: u16) {
        let group_no = group_no as usize - 1;
        if let Some(group) = self.get_group_mut(group_no) {
            let new_task = Task {
                description,
                is_done: false,
            };
            group.tasks.push(new_task)
        }
    }

    fn get_group_mut(&mut self, group_no: usize) -> Option<&mut TaskGroup> {
        match self.task_groups.get_mut(group_no as usize) {
            Some(task_group) => Some(task_group),
            None => {
                println!("Bad argument for `group number`, ignoring command.");
                None
            }
        }
    }

    fn get_task_mut(&mut self, group_no: u16, task_no: u16) -> Option<&mut Task> {
        let (task_no, group_no) = (task_no as usize - 1, group_no as usize - 1);
        let task_group = match self.get_group_mut(group_no) {
            Some(task_group_mut) => task_group_mut,
            None => return None,
        };

        match task_group.tasks.get_mut(task_no) {
            Some(task) => Some(task),
            None => {
                println!("Bad argument for `task number`, ignoring command.");
                None
            }
        }
    }

    fn flip_task(&mut self, group_no: u16, task_no: u16) {
        if let Some(task) = self.get_task_mut(group_no, task_no) {
            task.is_done = !task.is_done;
        }
    }

    fn remove_task(&mut self, group_no: u16, task_no: u16) {
        let (task_no, group_no) = (task_no as usize - 1, group_no as usize - 1);
        if let Some(group) = self.get_group_mut(group_no) {
            if task_no < group.tasks.len() {
                group.tasks.remove(task_no);
            }
        }
    }

    fn remove_group(&mut self, group_no: u16) {
        let group_no = group_no as usize - 1;
        if group_no < self.task_groups.len() {
            self.task_groups.remove(group_no);
        } else {
            println!("{}: group no. {} not found.", "error".red(), group_no + 1);
        }
    }

    pub fn evaluate(&mut self, command: Command) {
        use Command::*;
        match command {
            NoOp => {}
            AddTask(description, group) => {
                self.add_task(description, group);
            }
            AddGroup(group_name) => self.task_groups.push(TaskGroup {
                name: group_name,
                tasks: vec![],
            }),
            Command::FlipTask(group_no, task_no) => {
                self.flip_task(group_no, task_no);
            }
            Command::RemoveTask(group_no, task_no) => {
                self.remove_task(group_no, task_no);
            }
            Command::RemoveGroup(group_no) => {
                self.remove_group(group_no);
            }
            Command::ShowList => {
                print!("{}", self);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_task_groups() -> Vec<TaskGroup> {
        vec![
            TaskGroup {
                name: "Group 1".into(),
                tasks: vec![Task {
                    description: "Study for the Physics test".into(),
                    is_done: true,
                }],
            },
            TaskGroup {
                name: "College".into(),
                tasks: vec![Task {
                    description: "Study for the Maths test".into(),
                    is_done: true,
                }],
            },
        ]
    }

    #[test]
    fn flip_task_1() -> Result<(), errors::Error> {
        let mut todo_list = TodoList::try_from(sample_task_groups())?;

        let flip_task = Command::FlipTask(1, 1);
        let config_path = config_path::get_config_path()?;

        todo_list.evaluate(flip_task);

        assert_eq!(
            TodoList {
                task_groups: vec![
                    TaskGroup {
                        name: "Group 1".into(),
                        tasks: vec![Task {
                            description: "Study for the Physics test".into(),
                            is_done: false, // Flipped from true to false
                        },],
                    },
                    TaskGroup {
                        name: "College".into(),
                        tasks: vec![Task {
                            description: "Study for the Maths test".into(),
                            is_done: true,
                        },],
                    },
                ],
                config_path,
            },
            todo_list
        );

        Ok(())
    }

    #[test]
    fn flip_task_2() -> Result<(), errors::Error> {
        let mut todo_list = TodoList::try_from(sample_task_groups())?;

        let add_task = Command::AddTask("New task".into(), 1);
        let flip_task = Command::FlipTask(1, 2);
        let config_path = config_path::get_config_path()?;

        todo_list.evaluate(add_task);
        todo_list.evaluate(flip_task);

        assert_eq!(
            TodoList {
                task_groups: vec![
                    TaskGroup {
                        name: "Group 1".into(),
                        tasks: vec![
                            Task {
                                description: "Study for the Physics test".into(),
                                is_done: true,
                            },
                            Task {
                                description: "New task".into(),
                                is_done: true,
                            }
                        ],
                    },
                    TaskGroup {
                        name: "College".into(),
                        tasks: vec![Task {
                            description: "Study for the Maths test".into(),
                            is_done: true,
                        },],
                    },
                ],
                config_path,
            },
            todo_list
        );

        Ok(())
    }

    #[test]
    fn add_group() -> Result<(), errors::Error> {
        let mut todo_list = TodoList::try_from(sample_task_groups())?;
        let add_group_cmd = Command::AddGroup("New group".into());
        todo_list.evaluate(add_group_cmd);

        let config_path = config_path::get_config_path()?;

        assert_eq!(
            TodoList {
                task_groups: vec![
                    TaskGroup {
                        name: "Group 1".into(),
                        tasks: vec![Task {
                            description: "Study for the Physics test".into(),
                            is_done: true,
                        }],
                    },
                    TaskGroup {
                        name: "College".into(),
                        tasks: vec![Task {
                            description: "Study for the Maths test".into(),
                            is_done: true,
                        },],
                    },
                    TaskGroup {
                        name: "New group".into(),
                        tasks: vec![],
                    }
                ],
                config_path
            },
            todo_list
        );

        Ok(())
    }

    #[test]
    fn add_task() -> Result<(), errors::Error> {
        let mut todo_list = TodoList::try_from(sample_task_groups())?;

        let add_new_task = Command::AddTask("Sample new task".into(), 1);
        let config_path = config_path::get_config_path()?;

        todo_list.evaluate(add_new_task);

        assert_eq!(
            TodoList {
                task_groups: vec![
                    TaskGroup {
                        name: "Group 1".into(),
                        tasks: vec![
                            Task {
                                description: "Study for the Physics test".into(),
                                is_done: true,
                            },
                            Task {
                                description: "Sample new task".into(),
                                is_done: false,
                            }
                        ],
                    },
                    TaskGroup {
                        name: "College".into(),
                        tasks: vec![Task {
                            description: "Study for the Maths test".into(),
                            is_done: true,
                        },],
                    },
                ],
                config_path
            },
            todo_list
        );
        Ok(())
    }
}
