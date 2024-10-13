use crate::inner::arch_ins::memory::page::frame::{ ADDRESS_WIDTH, OFFSET_WIDTH };

pub const ADDRESS_MASK: usize = (1 << ADDRESS_WIDTH) - 1;
pub const OFFSET_MASK: usize = (1 << OFFSET_WIDTH) - 1;
pub const NUMBER_MASK: usize = (1 << (ADDRESS_WIDTH - OFFSET_WIDTH)) - 1;

pub fn init() {
    extern "C" {
        fn ekernel();
    }

    MANAGER.exclusive_access()
    .init(Address::from(ekernel as usize).ceil_number(), Address::from(crate::inner::arch_ins::memory::MEMORY_END).number());
}

#[derive(Clone, Copy)]
pub struct Address(pub usize);

impl From<usize> for Address {
    fn from(value: usize) -> Self {
        Self(value & ADDRESS_MASK)
    }
}

impl Into<usize> for Address {
    fn into(self) -> usize {
        self.0
    }
}

impl Address {
    #[inline]
    fn number(&self) -> Number {
        Number(self.0 >> OFFSET_WIDTH)
    }

    #[inline]
    fn offset(&self) -> usize {
        self.0 & OFFSET_MASK
    }
    
    #[inline]
    fn ceil_number(&self) -> Number {
        let frame_num_int: usize = self.number().into();
        
        if self.offset() == 0 {
            frame_num_int.into()
        } else {
            (frame_num_int + 1).into()
        }
    }
}

#[derive(Clone, Copy)]
pub struct Number(pub usize);

impl From<usize> for Number {
    fn from(value: usize) -> Self {
        Self(value & NUMBER_MASK)
    }
}

impl Into<usize> for Number {
    fn into(self) -> usize {
        self.0
    }
}

impl Number { 
    pub fn address(&self) -> Address {
        Address(self.0 << OFFSET_WIDTH)
    }
}

pub struct Frame {
    pub number: Number,
}

impl Frame {
    pub fn new() -> Result<Self, ()> {
        let alloc_result = MANAGER
        .exclusive_access()
        .alloc();

        match alloc_result {
            Ok(number) => Ok(Self { number }),
            Err(_) => Err(())
        }        
    }
}

// can't impl Drop for speci
impl Drop for Frame {
    fn drop(&mut self) {
        MANAGER
        .exclusive_access()
        .dealloc(self.number);
    }
}

use alloc::vec::Vec;

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

    pub fn dealloc(&mut self, number: Number) -> Result<(), ()>{
        let frame_num_int: usize = number.into();
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
        self.recycled.push(number);

        Ok(())
    }
}

use lazy_static::*;
use crate::sync::UPSafeCell;
lazy_static! {
    static ref MANAGER: UPSafeCell<Manager> = unsafe {
        UPSafeCell::new(Manager::new())
    };
}
