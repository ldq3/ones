mod frame;

use ones::virtualization::memory::page::{ RootTable, TableEntry, VirtualAddress };

pub type Table = RootTable<
    3,
    LocalEntry,
    LocalAddress,
>;

type LocalEntry = TableEntry<
    0b000_000_000,
    0b111_111_111,
>;

type LocalAddress = VirtualAddress<
    0b000_000_000,
    0b111_111_111,
>;