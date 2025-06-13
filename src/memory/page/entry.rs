use core::usize;

use crate::memory::Flag;

/**
public for test
*/
pub trait Lib {
    /**
    const FRAME_NUMBER_MASK: usize,
    const FLAG_MASK: usize,

    let frame_number_bits = frame_num << FRAME_NUMBER_MASK.trailing_zeros();
        let flag_bits = (page_flag.bits as usize) << FLAG_MASK.trailing_zeros();

        Entry(frame_number_bits | flag_bits)

        #[inline]
    fn frame_number(&self) -> usize {
        (self.0 & FRAME_NUMBER_MASK) >> FRAME_NUMBER_MASK.trailing_zeros()
    }

    fn flag(&self) -> Flag {
        let flag = (self.0 & FLAG_MASK) >> FLAG_MASK.trailing_zeros();

        Flag::from_bits(flag as u8).unwrap()
    }

    fn set_flag(&mut self, page_flag: Flag) {
        let frame_number_bits = self.0 & FRAME_NUMBER_MASK;
        let flag_bits = (page_flag.bits as usize) << FLAG_MASK.trailing_zeros();

        self.0 = frame_number_bits | flag_bits
    }
    */
    fn new(frame_num: usize, page_flag: Flag) -> Entry;
    fn frame_number(entry: &Entry) -> usize;
    fn flag(entry: &Entry) -> Flag;
    fn flag_set(entry: &mut Entry, page_flag: Flag);
}

/**Page Table Entry

假设 page table entry 由两部分组成:
- frame number
- flag
*/
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Entry(usize);

impl Entry {
    #[inline]
    pub fn bits(&self) -> usize {
        self.0
    }

    #[inline]
    pub fn from_bits(bits: usize) -> Self {
        Self(bits)
    }

    #[inline]
    pub fn bits_set(&mut self, bits: usize) {
        self.0 = bits
    }
}