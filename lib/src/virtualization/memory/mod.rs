pub mod page;
pub mod segment;

mod arch;

pub struct OriginalAddress(usize);

impl From<usize> for OriginalAddress {
    fn from(value: usize) -> Self {
        OriginalAddress(value)
    }
}

impl Into<usize> for OriginalAddress {
    fn into(self) -> usize {
        self.0
    }
}

pub fn init() {
    
}

pub use arch::riscv64::*;

// address translation
// protection: read/write/execute