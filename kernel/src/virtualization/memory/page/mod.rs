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

mod frame;

pub fn init() {
    use ones::virtualization::memory::page::frame as lib_frame;
    use crate::virtualization::memory;

    lib_frame::init(memory::config::END, memory::config::END).unwrap();
}

use ones::virtualization::memory::page::{ RootTable, TableEntry, Address };

pub type Table = RootTable<
    3,
    LocalEntry,
    VirtualAddress,
>;

type LocalEntry = TableEntry<
    0b111_111_111_111_111_111_111_111_111_111_111_111_111_111_110_000_000_000,
    0b11_111_111_111,
>;

type VirtualAddress = Address<
    0b111_111_111_111_111_111_111_111_111_000_000_000_000,
    0b000_000_000_000,
>;