use alloc::collections::vec_deque::VecDeque;

pub struct Semaphore<T> {
    resources: i32,
    blocked_tasks: VecDeque<T>,
}

impl<T> Semaphore<T> {
    pub fn new(resources: u32) -> Self {
        Self {
            resources: resources.cast_signed(),
            blocked_tasks: VecDeque::new(),
        }
    }

    pub fn take(&mut self, task: T) {
        if self.resources <= 0 {
            self.blocked_tasks.push_back(task);
            // Suspend task
        }
        self.resources -= 1;
    }

    pub fn free(&mut self, task: T) {
        self.resources += 1;
        if let Some(blocked_task) = self.blocked_tasks.pop_front() {
            // Wake task
        }
    }
}
