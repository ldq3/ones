
// config
pub const ADDRESS_WIDTH: usize = 39; // SV 39
pub const OFFSET_WIDTH: usize = 12;
pub const LEVEL: usize = 3;

use crate::inner::memory::page::Flags;
pub const FLAGS: Flags = Flags {
    valid: 1,
    read: 1 << 1,
    write: 1 << 2,
    execute: 1 << 3,
    user: 1 << 4,
    global: 1 << 5,
    accessed: 1 << 6,
    dirty: 1 << 7,
};

pub use crate::inner::memory::page::{
    self, Address, Number
};
