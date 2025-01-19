mod arch;

use lazy_static::*;
use crate::concurrency::UPSafeCell;
lazy_static! {
    static ref MANAGER: UPSafeCell<Manager> = unsafe {
        UPSafeCell::new(Manager::new())
    };
}

pub struct Address;

impl Address {
    #[inline]
    pub fn guard(value: usize) -> Result<(), ()> {
        if (value & !config::ADDRESS_MASK) != 0 {
            Err(()) 
        } else {
            Ok(())
        }
    }

    pub fn offset(address: usize) -> usize {
        address & config::OFFSET_MASK
    }

    pub fn number(address: usize) -> usize {
        address >> config::OFFSET_WIDTH
    }
    
    pub fn ceil_number(address: usize) -> usize {        
        if Self::offset(address) == 0 {
            Self::number(address)
        } else {
            Self::number(address) + 1
        }
    }
}

pub struct Frame{
    pub number: usize
} // FIXME: 如何纳入 rust 所有权系统？不提供直接使用 frame number 的接口

// can't impl Drop for speci
impl Drop for Frame {
    fn drop(&mut self) {
        MANAGER.exclusive_access().dealloc(self).unwrap();
    }
}

impl Frame {
    /**
    检查页框号形式是否合法
    */
    #[inline]
    fn guard(value: usize) -> Result<(), ()> {
        if (value & !config::NUMBER_MASK) != 0 {
            Err(())
        } else {
            Ok(())
        }
    }

    /**
    # 返回
    页框的物理地址
    */
    pub fn address(&self) -> usize {
        self.number << config::OFFSET_WIDTH
    }
}

/**
the range [current, end) represents physical page numbers that have never been allocated before

the stack 'recycled' saves the physical page numbers that have been deallocated after being allocated.
*/
struct Manager {
    start: Option<usize>, // 无所有权
    length: usize, // FIXME: 如何保证 length 的合法性？
    recycled: Vec<usize>, // 为实现 Drop，不可用 Frame
}

impl Manager {
    fn new() -> Self {
        // FIXME: mutref and mut?

        Manager {
            start: None,
            length: 0,
            recycled: Vec::new(),
        }
    }

    fn init(&mut self, start: usize, length: usize) -> Result<(), ()> {
        match self.start {
            Some(_) => Err(()),
            None => {
                self.start = Some(start);
                self.length = length;

                Ok(())
            }
        }
    }

    fn alloc(&mut self) -> Result<Frame, ()> {
        if let Some(number) = self.recycled.pop() {
            Ok(Frame{ number })
        } else {
            if self.length == 0 {
                Err(())
            } else {
                if let Some(start) = self.start {
                    self.length -= 1;

                    Ok(Frame{ number: start + self.length })
                } else {
                    Err(())
                }
            }
        }
    }

    fn dealloc(&mut self, frame: &mut Frame) -> Result<(), ()>{
        if let Some(start) = self.start {
            if frame.number < start + self.length
            || self.recycled.iter().find(|&v| { *v == frame.number }).is_some() { // 未分配
                return Err(())
            }
            
            self.recycled.push(frame.number);

            Ok(())  
        } else {
            Err(())
        } 
    }
}

pub fn init(start: usize, length: usize) -> Result<(), ()> {
    if let Err(()) = Frame::guard(start) { return Err(()); }

    MANAGER.exclusive_access().init(start, length)
}

pub fn alloc_frame() -> Result<Frame, ()> {
    match MANAGER.exclusive_access().alloc() {
        Ok(number) => Ok(number),
        Err(_) => Err(())
    }  
}

mod config {
    pub use super::arch::riscv64::*;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn main() {
        init(0, 1).unwrap();

        if let Ok(_) = alloc_frame() {
            assert!(true)
        } else {
            assert!(false)
        }

        if let Ok(_) = alloc_frame() {
            if let Err(_) = alloc_frame() {
                assert!(true)
            } else {
                assert!(false)
            }
        } else {
            assert!(false)
        }
    }
}