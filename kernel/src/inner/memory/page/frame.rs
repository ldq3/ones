use alloc::vec::Vec;

use super::FrameNum;

/**
the range [current, end) represents physical page numbers that have never been allocated before

the stack 'recycled' saves the physical page numbers that have been deallocated after being allocated.
*/
pub struct Manager<F: FrameNum> {
    current: F,
    end: F,
    recycled: Vec<F>,
}

impl<F: FrameNum> Manager<F> {
    pub fn init(&mut self, l:F, r: F) {
        self.current = l;
        self.end = r;
    }

    pub fn new() -> Self {
        Self {
            current: 0.into(),
            end: 0.into(),
            recycled: Vec::new(),
        }
    }

    pub fn alloc(&mut self) -> Result<F, ()> {
        if let Some(frame_num) = self.recycled.pop() {
            Ok(frame_num)
        } else {
            let mut current_int: usize = self.current.into();
            let mut end_int: usize = self.end.into();

            if current_int == end_int {
                Err(())
            } else {
                current_int += 1;
                self.current = (current_int).into();
                Ok((current_int - 1).into())
            }
        }
    }

    pub fn dealloc(&mut self, frame_num: F) -> Result<(), ()>{
        let frame_num_int: usize = frame_num.into();
        let current_int: usize = self.current.into();

        // validity check
        if frame_num_int >= current_int || self.recycled
            .iter()
            .find(|&v| {(*v).into() == frame_num_int})
            .is_some() {
            // frame has not been allocated
            return Err(())
        }
        // recycle
        self.recycled.push(frame_num);

        Ok(())
    }
}