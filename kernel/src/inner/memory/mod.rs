
pub mod heap;
pub mod page;
pub mod segment;

pub trait OriginalAddress: Sized {
    fn new(n: usize) -> Result<Self, ()>;
}

// address translation
// protection: read/write/execute