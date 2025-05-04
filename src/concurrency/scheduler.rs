use alloc::collections::vec_deque::VecDeque;

use crate::Allocator;

/**
ready: (process id, thread id)
*/
pub struct Preemptive {
    pub running:   Option<usize>,
    pub   ready: VecDeque<usize>,
    pub blocked: VecDeque<usize>,

    pub allocator: Allocator
}

impl Preemptive {
    pub fn new(cap: usize) -> Self {
        Self {
            running: None,
            ready: VecDeque::new(),
            blocked: VecDeque::new(),

            allocator: Allocator::new(1, cap - 1).unwrap()
        }
    }

    pub fn add(&mut self) -> usize {
        let id = self.allocator.alloc().unwrap();

        self.ready.push_back(id);

        id
    }

    pub fn terminate(&mut self) -> usize {
        let id = self.running.unwrap();
        
        self.running = None;
        
        id
    }

    pub fn switch(&mut self) {
        let current = self.running.unwrap();
        let next = self.ready.pop_front().unwrap();

        self.running = Some(next);
        self.blocked.push_back(current);
    }

    pub fn preempt(&mut self) {
        let current = self.running.unwrap();
        let next = self.ready.pop_front().unwrap();

        self.running = Some(next);
        self.ready.push_back(current);
    }
}
