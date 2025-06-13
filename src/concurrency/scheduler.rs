use alloc::{collections::vec_deque::VecDeque, vec::Vec};

use crate::Allocator;

/**
ready: (process id, thread id)
*/
pub struct Scheduler {
    pub running:   Option<usize>,
    pub ready:     VecDeque<usize>,
    pub blocked:   VecDeque<usize>,
    pub completed: Vec<usize>,

    pub allocator: Allocator
}

impl Scheduler {
    /**
    The id 0 is special.
    */
    pub fn new(cap: usize) -> Self {
        Self {
            running: None,
            ready: VecDeque::new(),
            blocked: VecDeque::new(),
            completed: Vec::new(),

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
    /**
    返回被调度的 id
    */
    pub fn switch_s(&mut self) -> usize {
        let next = self.ready.pop_front().unwrap();
        self.running = Some(next);

        if let Some(current) = self.running {
            self.blocked.push_back(current);
        }

        next
    }
    /**
    返回被调度的 id
    */
    pub fn preempt(&mut self) -> usize {
        let next = self.ready.pop_front().unwrap();
        self.running = Some(next);

        if let Some(current) = self.running {
            self.ready.push_back(current);
        }

        next
    }
}
