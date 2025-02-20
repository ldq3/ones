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

use ones::virtualization::memory::page::{ ModelTable, ModelTableEntry, ModelAddress };

pub type Table = ModelTable<
    3,
    TableEntry,
    VirtualAddress,
>;

type TableEntry = ModelTableEntry<
    0b111_111_111_111_111_111_111_111_111_111_111_111_111_111_110_000_000_000,
    0b11_111_111_111,
>;

pub type VirtualAddress = ModelAddress<
    0b111_111_111_111_111_111_111_111_111_000_000_000_000,
    0b000_000_000_000,
>;

use ones::virtualization::memory::page::Table as _;
use lazy_static::lazy_static;
use spin::Mutex;
lazy_static!{
    pub static ref KERNEL_TABLE: Mutex<Table> = Mutex::new(Table::new());
}
