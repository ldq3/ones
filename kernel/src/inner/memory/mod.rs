
pub mod page;
pub mod segment;

mod arch;

pub trait OriginalAddress: Sized {
    fn new(n: usize) -> Result<Self, ()>;
}

pub use arch::riscv64::*;

// address translation
// protection: read/write/execute