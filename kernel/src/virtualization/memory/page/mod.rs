/*!
# SV 39
## 虚拟地址
const ADDRESS_WIDTH: usize = 39;

const NUMBER_WIDTH: usize = 27;

const OFFSET_WIDTH: usize = 12;

## 页表项
const FRAME_NUMER_WIDTH; // 参见 frame

const RSW_WIDTH: usize = 2;

const FLAG_WIDTH: usize = 8;
*/

pub mod frame;

use ones::virtualization::memory::page::{ Table, TableEntry, Address };

pub use ones::virtualization::memory::page::frame::init;

pub type LocalTable = Table<
    3,
    LocalTableEntry,
    VirtualAddress,
>;

type LocalTableEntry = TableEntry<
    0b111_111_111_111_111_111_111_111_111_111_111_111_111_111_110_000_000_000,
    0b11_111_111_111,
>;

type VirtualAddress = Address<
    0b111_111_111_111_111_111_111_111_111_000_000_000_000,
    0b000_000_000_000,
>;

use lazy_static::*;
use spin::Mutex;
lazy_static!{
    pub static ref KERNEL_PAGE_TABLE: Mutex<LocalTable> = Mutex::new(LocalTable::new());
}
