use crate::inner::arch_ins::memory::page::frame;

use crate::sync::UPSafeCell;
use alloc::vec::Vec;
use lazy_static::*;

lazy_static! {
    static ref FRAME_MANAGER: UPSafeCell<Manager> = unsafe {
        UPSafeCell::new(Manager::new())
    };
}

pub fn init() {
    extern "C" {
        fn ekernel();
    }
    FRAME_MANAGER
        .exclusive_access()
        .init(frame::PhysicalAddressRv64::from(ekernel as usize).ceil_frame_num(), frame::PhysicalAddressRv64::from(crate::inner::arch_ins::memory::MEMORY_END).frame_num());
}

#[derive(Clone, Copy)]
pub struct Number(pub usize);

pub trait NumberOperation: Into<usize> + From<usize> {
    type PhysicalAddress: PhysicalAddress;

    fn physical_address(&self) -> Self::PhysicalAddress;
}

pub trait PhysicalAddress: From<usize> + Into<usize> {
    fn frame_num(&self) -> Number;

    fn offset(&self) -> usize;

    #[inline]
    fn ceil_frame_num(&self) -> Number {
        let frame_num_int: usize = self.frame_num().into();
        
        if self.offset() == 0 {
            frame_num_int.into()
        } else {
            (frame_num_int + 1).into()
        }
    }
}

/**
the range [current, end) represents physical page numbers that have never been allocated before

the stack 'recycled' saves the physical page numbers that have been deallocated after being allocated.
*/
pub struct Manager {
    current: Number,
    end: Number,
    recycled: Vec<Number>,
}

impl Manager {
    pub fn init(&mut self, l:Number, r: Number) {
        self.current = l;
        self.end = r;
    }

    pub fn new() -> Self {
        Self {
            current: Number(0),
            end: Number(0),
            recycled: Vec::new(),
        }
    }

    pub fn alloc(&mut self) -> Result<Number, ()> {
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

    pub fn dealloc(&mut self, frame_num: Number) -> Result<(), ()>{
        let frame_num_int: usize = frame_num.into();
        let current_int: usize = self.current.into();

        // validity check
        if frame_num_int >= current_int || self.recycled
            .iter()
            .find(|&v| {
                let v_int: usize = (*v).into(); // FIXME: how to use into?
                v_int == frame_num_int
            })
            .is_some() {
            // frame has not been allocated
            return Err(())
        }
        // recycle
        self.recycled.push(frame_num);

        Ok(())
    }
}

pub struct Frame {
    pub ppn: Number,
}

impl Frame {
    pub fn new() -> Result<Self, ()> {
        let res_alloc = FRAME_MANAGER
        .exclusive_access()
        .alloc();

        match res_alloc {
            Ok(ppn) => Ok(Self { ppn }),
            Err(_) => Err(())
        }        
    }
}

// can't impl Drop for speci
impl Drop for Frame {
    fn drop(&mut self) {
        FRAME_MANAGER
        .exclusive_access()
        .dealloc(self.ppn);
    }
}