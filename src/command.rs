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