const FRAME_SIZE: usize = 4096; // 4 KiB
const PAGE_SIZE: usize = FRAME_SIZE;
use super::MEMORY_END;
const PAGE_SIZE_BITS: usize = 3;
const PA_WIDTH_SV39: usize = 56;
const VA_WIDTH_SV39: usize = 39;
const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_SIZE_BITS;

pub mod table;

use alloc::vec;
use alloc::vec::Vec;
use riscv::addr::Page;
use crate::sync::UPSafeCell;
use lazy_static::*;

#[derive(Clone, Copy)]
pub struct FrameNumRv(usize);

use crate::inner::memory::page::FrameNum;

impl From<usize> for FrameNumRv {
    fn from(value: usize) -> Self {
        Self(value & ( (1 << PPN_WIDTH_SV39) - 1 ))
    }
}

impl Into<usize> for FrameNumRv {
    fn into(self) -> usize {
        self.0
    }
}

impl FrameNum for FrameNumRv { 
    type PhysicalAddress = PhysicalAddressRv64;

    fn physical_address(&self) -> Self::PhysicalAddress {
        PhysicalAddressRv64(self.0 << PAGE_SIZE_BITS)
    }
}

pub struct PhysicalAddressRv64(usize);

impl From<usize> for PhysicalAddressRv64 {
    fn from(value: usize) -> Self {
        Self(value & ( (1 << PA_WIDTH_SV39) - 1 ))
    }
}

impl Into<usize> for PhysicalAddressRv64 {
    fn into(self) -> usize {
        self.0
    }
}

use crate::inner::memory::page::PhysicalAddress;
impl PhysicalAddress for PhysicalAddressRv64 {
    type F = FrameNumRv;

    #[inline]
    fn frame_num(&self) -> FrameNumRv {
        self.0.into()
    }

    #[inline]
    fn offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
}

pub struct FrameRV {
    pub ppn: FrameNumRv,
}

use crate::inner::memory::page::Frame;
impl Frame for FrameRV {
    fn new() -> Result<Self, ()> {
        let res_alloc = FRAME_MANAGER
        .exclusive_access()
        .alloc();

        match res_alloc {
            Ok(ppn) => Ok(Self { ppn }),
            Err(_) => Err(())
        }        
    }
}

impl Drop for FrameRV {
    fn drop(&mut self) {
        FRAME_MANAGER
        .exclusive_access()
        .dealloc(self.ppn);
    }
}

pub fn init() {
    extern "C" {
        fn ekernel();
    }
    FRAME_MANAGER
        .exclusive_access()
        .init(PhysicalAddressRv64::from(ekernel as usize).ceil_frame_num(), PhysicalAddressRv64::from(MEMORY_END).frame_num());
}

use crate::inner::memory::page::frame::Manager as FrameManager; 

lazy_static! {
    static ref FRAME_MANAGER: UPSafeCell<FrameManager<FrameNumRv>> = unsafe {
        UPSafeCell::new(FrameManager::new())
    };
}

#[derive(Clone, Copy)]
pub struct PageNumRv39(usize);
pub struct VirtualAddressRv39(usize);

use crate::inner::memory::page::{
    PageNum,
    VirtualAddress
};

impl From<usize> for PageNumRv39 {
    fn from(value: usize) -> Self {
        Self(value & ((1 << PPN_WIDTH_SV39) - 1))
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
