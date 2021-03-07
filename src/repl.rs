use crate::task::{Task, TaskGroup};


/// dodo's Read-Eval-Print Loop
pub struct REPL {
    task_groups: Vec<TaskGroup>
}

impl REPL {
    pub fn new(task_groups: Vec<TaskGroup>) -> Self {
        Self {
            task_groups
        }
    }
}