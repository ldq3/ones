
// config
const ADDRESS_WIDTH: usize = 56; // SV 39
pub const OFFSET_WIDTH: usize = 12; 

pub const ADDRESS_MASK: usize = (1 << ADDRESS_WIDTH) - 1;
pub const OFFSET_MASK: usize = (1 << OFFSET_WIDTH) - 1;
pub const NUMBER_MASK: usize = (1 << (ADDRESS_WIDTH - OFFSET_WIDTH)) - 1;
