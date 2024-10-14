
// config
pub const ADDRESS_WIDTH: usize = 39; // SV 39
pub const OFFSET_WIDTH: usize = 12;
pub const LEVEL: usize = 3;

pub mod frame;

pub use crate::inner::memory::page::{
    Address, Number
};

use crate::inner::memory::page::TableEntryFlag;

use bitflags::*;
bitflags! {
    pub struct FlagsRv: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}

impl TableEntryFlag for FlagsRv {
    fn bits(&self) -> u8 {
        0
    }

    fn from_bits(bits: u8) -> Self {
        Self::from_bits_truncate(bits)
    }

    fn readable(&self) -> bool {
        self.contains(FlagsRv::R)
    }
    fn writable(&self) -> bool {
        self.contains(FlagsRv::W)
    }
    fn executable(&self) -> bool {
        self.contains(FlagsRv::X)
    }
}