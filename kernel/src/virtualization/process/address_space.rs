// config
pub const GLOBAL_CODE_PAGE_NUMBER: usize = 0; // usize::MAX - PAGE_SIZE + 1;
pub const GLOBAL_DATA_PAGE_NUMBER: usize = 0; // TRAMPOLINE - PAGE_SIZE;

use ones::virtualization::{
    memory::page::Map,
    KernelAddressSpace, Operation
};
struct Wrapper(KernelAddressSpace);

impl Operation for Wrapper {
    fn init(&self) {
        use crate::virtualization::memory::page;
        let mut page_table = page::Table::new().unwrap();

        for (segment, map) in &self.0.0 {
            if let Map::Identical = map {
                for page_num in segment.range.0..segment.range.1 {
                    page_table.identical_map(page_num, segment.flag).unwrap();
                }
            } else {
                for page_num in segment.range.0..segment.range.1 {
                    page_table.insert(page_num, segment.flag).unwrap();
                }
            }
        }

        use riscv::register::satp;
        use core::arch::asm;
        unsafe {
            satp::write(page_table.root.1.number); // 8usize << 60 | self.root_ppn.0
            asm!("sfence.vma");
        }
    }
}
