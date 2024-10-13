use crate::inner::memory::page::frame;

// config
pub const ADDRESS_WIDTH: usize = 56; // SV 39
pub const OFFSET_WIDTH: usize = 12; 

pub use frame::Number;
pub use frame::Address;
pub use frame::Frame;