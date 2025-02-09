/*！
# SV 39
RV 64

开启虚拟内存后：
有效物理内存地址为 56 位
虚拟内存地址为 64 位，但仅低 39 位有效，共分为三层页表

satp 寄存器的组成：

| 位域 | 位数 | 描述                                                                 |
|------|------|----------------------------------------------------------------------|
| MODE | 1-3  | 地址转换模式：值为 1 表示使用 Sv39 模式                       |
| ASID | 4-11 | 地址空间标识符（Address Space Identifier），用于区分不同的地址空间 |
| PPN  | 12-63| 物理页号（Physical Page Number），指向顶级页表的物理地址           |
*/

mod page;

pub fn init() {
    use ones::virtualization::memory::{ KernelAddressSpace, page::Map };

    page::init();

    use crate::virtualization::memory::page;
    let mut page_table = page::Table::new().unwrap();

    #[allow(unused)]
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss_with_stack();
        fn ebss();
        fn ekernel();
        fn strampoline();
    }

    let space = KernelAddressSpace::new(
        (stext as usize, etext as usize),
        (srodata as usize, erodata as usize),
        (sdata as usize, edata as usize),
        (sbss_with_stack as usize, ebss as usize),
        (ekernel as usize , ekernel as usize),
        (strampoline as usize, strampoline as usize)
    );

    for (segment, map) in space.into_iter() {
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

mod config {
    pub const END: usize = 0x80_800_000;
}
