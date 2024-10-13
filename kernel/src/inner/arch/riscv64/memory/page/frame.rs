use crate::inner::memory::page::frame;

pub mod config {
    use super::super::{
        PAGE_SIZE,
        PAGE_SIZE_BITS
    };

    pub const PA_WIDTH_SV39: usize = 56;
    pub const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_SIZE_BITS;
    pub const FRAME_SIZE: usize = PAGE_SIZE;
    pub const FREME_SIZE_BITS: usize = PAGE_SIZE_BITS;
}

pub use frame::Number;

impl From<usize> for Number {
    fn from(value: usize) -> Self {
        Self(value & ( (1 << config::PPN_WIDTH_SV39) - 1 ))
    }
}

impl Into<usize> for Number {
    fn into(self) -> usize {
        self.0
    }
}

impl frame::NumberOperation for Number { 
    type PhysicalAddress = PhysicalAddressRv64;

    fn physical_address(&self) -> Self::PhysicalAddress {
        PhysicalAddressRv64(self.0 << config::FREME_SIZE_BITS)
    }
}

pub struct PhysicalAddressRv64(usize);

impl From<usize> for PhysicalAddressRv64 {
    fn from(value: usize) -> Self {
        Self(value & ( (1 << config::PA_WIDTH_SV39) - 1 ))
    }
}

impl Into<usize> for PhysicalAddressRv64 {
    fn into(self) -> usize {
        self.0
    }
}

impl frame::PhysicalAddress for PhysicalAddressRv64 {
    #[inline]
    fn frame_num(&self) -> Number {
        self.0.into()
    }

    #[inline]
    fn offset(&self) -> usize {
        self.0 & (config::FRAME_SIZE - 1)
    }
}
