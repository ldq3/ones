const FRAME_SIZE: usize = 4096; // 4 KiB
const PAGE_SIZE: usize = FRAME_SIZE;
use super::MEMORY_END;
const PAGE_SIZE_BITS: usize = 12;
const VA_WIDTH_SV39: usize = 39;
const VPN_WIDTH_SV39: usize = VA_WIDTH_SV39 - PAGE_SIZE_BITS;

pub mod table;
pub mod frame;

use alloc::vec;
use alloc::vec::Vec;
use riscv::addr::Page;

use crate::inner::memory::page::frame::Manager as FrameManager; 

#[derive(Clone, Copy)]
pub struct PageNumRv39(usize);
pub struct VirtualAddressRv39(usize);

use crate::inner::memory::page::{
    PageNum,
    VirtualAddress
};

impl From<usize> for PageNumRv39 {
    fn from(value: usize) -> Self {
        Self(value & ((1 << VPN_WIDTH_SV39) - 1))
    }
}

impl Into<usize> for PageNumRv39 {
    fn into(self) -> usize {
        self.0
    }
}

impl PageNum for PageNumRv39 {}

impl From<usize> for VirtualAddressRv39 {
    fn from(value: usize) -> Self {
        Self(value & ((1 << VA_WIDTH_SV39) - 1))
    }
}

impl Into<usize> for VirtualAddressRv39 {
    fn into(self) -> usize {
        self.0
    }
}

impl VirtualAddress for VirtualAddressRv39 {
    type P = PageNumRv39;

    #[inline]
    fn page_num(&self) -> Self::P {
        PageNumRv39((self.0 / PAGE_SIZE))
    }

    #[inline]
    fn offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
}
