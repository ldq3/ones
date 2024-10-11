pub const MEMORY_END: usize = 0x80_800_000;

// address 56
pub mod page;

use crate::inner::memory::OriginalAddress;
pub struct OriginalAddressRv64(u64);

impl OriginalAddress for OriginalAddressRv64 {
    fn new(n: usize) -> Result<Self, ()> {
        Ok(Self(n as u64))
    }
}