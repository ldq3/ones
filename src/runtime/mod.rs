/*!
block started by symbol, bss

heap
*/

pub mod address_space;

use crate::memory::Flag;

/**
range 为页号

grouth true 代表向高地址方向增长，false 代表向低地址方向增长
*/
#[derive(Clone, Copy)]
pub struct Segment {
    pub range: (usize, usize),
    pub flag: Flag,
}

pub trait Runtime {
    fn init();
    fn clear_bss();
}