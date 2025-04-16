use core::usize;

use crate::memory::Flag;

/**
public for test
*/
pub trait Entry {
    fn new(frame_num: usize, page_flag: Flag) -> Self;
    fn frame_number(&self) -> usize;
    fn flag(&self) -> Flag;
    fn set_flag(&mut self, page_flag: Flag);
    fn bits(&self) -> usize;
    fn from_bits(bits: usize) -> Self;
}

/**Page Table Entry

假设 page table entry 由两部分组成:
- frame number
- flag
*/
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ModelEntry<
    const FRAME_NUMBER_MASK: usize,
    const FLAG_MASK: usize,
>(usize);

impl<
    const FRAME_NUMBER_MASK: usize,
    const FLAG_MASK: usize,
> Entry for ModelEntry<FRAME_NUMBER_MASK, FLAG_MASK> {
    fn new(frame_num: usize, page_flag: Flag) -> Self {
        let frame_number_bits = frame_num << FRAME_NUMBER_MASK.trailing_zeros();
        let flag_bits = (page_flag.bits as usize) << FLAG_MASK.trailing_zeros();

        ModelEntry(frame_number_bits | flag_bits)
    }

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

    #[inline]
    fn bits(&self) -> usize {
        self.0
    }

    #[inline]
    fn from_bits(bits: usize) -> Self {
        Self(bits)
    }
}