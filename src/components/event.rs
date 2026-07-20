use crate::components::task::{Task, TaskResult};

pub enum Event {
    TaskFinished(TaskResult),
    Timeout(u32),
    NewTask(Task),
}
