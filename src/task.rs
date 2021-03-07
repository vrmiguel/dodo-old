use std::{fmt, io::BufWriter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
/// Represents a task and wether or not it's been concluded
pub struct Task {
    pub description: String,
    pub is_done: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
/// A group of tasks
pub struct TaskGroup {
    /// The group's name
    pub name: String,
    /// The tasks belonging to this group
    pub tasks: Vec<Task>,
}

impl fmt::Display for TaskGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.name)?;

        for (i, task) in self.tasks.iter().enumerate() {
            writeln!(
                f,
                "\t{}. [{}] - {}",
                i + 1,
                if task.is_done { "x" } else { " " },
                task.description
            )?;
        }

        write!(f, "")
    }
}

#[cfg(test)]
/// Testing RON serialization and deserialization
mod ron_tests {
    use super::*;

    #[test]
    fn task_group_to_string() {
        let groups: Vec<TaskGroup> = vec![sample_group_1(), sample_group_2()];
        let groups = groups
            .iter()
            .rfold("".to_string(), |res, task| format!("{}{}", res, task));

        assert_eq!(
            groups,
            "Uni\n\t1. [x] - Study for the Physics test\n\t2. [x] - Study Monads\n\t3. [ ] - Finish the Compilers project\nChores\n\t1. [x] - Clean the house\n\t2. [x] - Unlearn JavaScript\n\t3. [ ] - Make Python statically typed\n"
        )
    }

    fn sample_group_1() -> TaskGroup {
        TaskGroup {
            name: "Chores".into(),
            tasks: vec![
                Task {
                    description: "Clean the house".into(),
                    is_done: true,
                },
                Task {
                    description: "Unlearn JavaScript".into(),
                    is_done: true,
                },
                Task {
                    description: "Make Python statically typed".into(),
                    is_done: false,
                },
            ],
        }
    }

    fn sample_group_2() -> TaskGroup {
        TaskGroup {
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
        }
    }

    #[test]
    fn group_serialization_1() {
        use ron;

        let serialized_data = ron::ser::to_string(&sample_group_1());

        assert!(serialized_data.is_ok());
        let serialized_data = serialized_data.unwrap();

        assert_eq!(
            serialized_data, 
            "(name:\"Chores\",tasks:[(description:\"Clean the house\",is_done:true),(description:\"Unlearn JavaScript\",is_done:true),(description:\"Make Python statically typed\",is_done:false)])"
        );
    }

    #[test]
    fn group_serialization_2() {
        let group = sample_group_2();
        let serialized_data = ron::ser::to_string(&group).unwrap();

        assert_eq!(
            serialized_data,
            "(name:\"Uni\",tasks:[(description:\"Study for the Physics test\",is_done:true),(description:\"Study Monads\",is_done:true),(description:\"Finish the Compilers project\",is_done:false)])"
        )
    }

    #[test]
    fn group_deserialization_1() {
        let serialized_data = "(name:\"Chores\",tasks:[(description:\"Clean the house\",is_done:true),(description:\"Unlearn JavaScript\",is_done:true),(description:\"Make Python statically typed\",is_done:false)])";

        let deserialized_group: TaskGroup =
            ron::de::from_str(serialized_data).expect("Deserialization failed!");

        assert_eq!(sample_group_1(), deserialized_group)
    }

    #[test]
    fn group_deserialization_2() {
        let serialized_data = "(name:\"Uni\",tasks:[(description:\"Study for the Physics test\",is_done:true),(description:\"Study Monads\",is_done:true),(description:\"Finish the Compilers project\",is_done:false)])";

        let deserialized_group: TaskGroup =
            ron::de::from_str(serialized_data).expect("Deserialization failed!");

        assert_eq!(sample_group_2(), deserialized_group)
    }

    #[test]
    fn group_vec_serialization() {
        let groups = vec![sample_group_1(), sample_group_2()];
        let serialized_data = ron::ser::to_string(&groups).unwrap();

        assert_eq!(
            "[(name:\"Chores\",tasks:[(description:\"Clean the house\",is_done:true),(description:\"Unlearn JavaScript\",is_done:true),(description:\"Make Python statically typed\",is_done:false)]),(name:\"Uni\",tasks:[(description:\"Study for the Physics test\",is_done:true),(description:\"Study Monads\",is_done:true),(description:\"Finish the Compilers project\",is_done:false)])]",
            serialized_data
        );
    }

    #[test]
    fn group_vec_deserialization() {
        let serialized_data = "[(name:\"Chores\",tasks:[(description:\"Clean the house\",is_done:true),(description:\"Unlearn JavaScript\",is_done:true),(description:\"Make Python statically typed\",is_done:false)]),(name:\"Uni\",tasks:[(description:\"Study for the Physics test\",is_done:true),(description:\"Study Monads\",is_done:true),(description:\"Finish the Compilers project\",is_done:false)])]";
        let deserialized_group: Vec<TaskGroup> = ron::de::from_str(serialized_data).unwrap();

        assert_eq!(deserialized_group, vec![sample_group_1(), sample_group_2()]);
    }
}
