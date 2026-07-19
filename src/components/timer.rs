use std::time::SystemTime;
use std::cmp::Ordering;


#[derive(Eq, PartialEq)]
pub struct Deadline {
    when: SystemTime,
    pub task_id: u32,
}

impl Deadline {
    pub fn new(task_id: u32, timeout: u64) -> Self {
        Self {
            task_id: task_id,
            when: SystemTime::now() + std::time::Duration::from_millis(timeout),
        }
    }

    pub fn is_expired(&self) -> bool {
        if self.when <= SystemTime::now() {
            return true;
        } else {
            return false;
        }
    }
}

impl Ord for Deadline {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order: earliest deadline becomes the heap top
        other.when.cmp(&self.when)
    }
}

impl PartialOrd for Deadline {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
