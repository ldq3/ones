pub const ADDRESS_MASK: usize = (1 << ADDRESS_WIDTH) - 1;
pub const OFFSET_MASK: usize = (1 << OFFSET_WIDTH) - 1;
pub const NUMBER_MASK: usize = (1 << (ADDRESS_WIDTH - OFFSET_WIDTH)) - 1;

const ADDRESS_WIDTH: usize = 39; // SV 39
pub const OFFSET_WIDTH: usize = 12;
pub const LEVEL: usize = 3;

// Flag
pub const VALID:    u8 = 1;
pub const READ:     u8 = 1 << 1;
pub const WRITE:    u8 = 1 << 2;
pub const EXECUTE:  u8 = 1 << 3;
pub const USER:     u8 = 1 << 4;
pub const GLOBAL:   u8 = 1 << 5;
pub const ACCESSED: u8 = 1 << 6;
pub const DIRTY:    u8 = 1 << 7;
