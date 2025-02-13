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

use log::info;
use ones::virtualization::memory::page::AddressTrait;

mod page;

pub fn init() {
    use ones::virtualization::memory::{ KernelAddressSpace, page::Map };

    use crate::virtualization::memory::page::{ self, frame::PhysicalAddress };

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

    let start = PhysicalAddress::ceil_number(ekernel as usize);
    let length = PhysicalAddress::number(config::END) - start;

    page::init(start, length);
    info!("Frame manager is initialized at: {:x}, length: {:x}", start, length);

    let mut page_table = page::LocalTable::new();

    let text = (PhysicalAddress::number(stext as usize), PhysicalAddress::number(etext as usize));
    let read_only_data = (PhysicalAddress::number(srodata as usize), PhysicalAddress::number(erodata as usize));
    let data = (PhysicalAddress::number(sdata as usize), PhysicalAddress::number(edata as usize));
    let static_data = (PhysicalAddress::number(sbss_with_stack as usize), PhysicalAddress::number(ebss as usize));
    let trap_code = PhysicalAddress::number(strampoline as usize);

    info!("Segement text: {:x} - {:x}", text.0, text.1);
    info!("Segement read only data: {:x} - {:x}", read_only_data.0, read_only_data.1);
    info!("Segement data: {:x} - {:x}", data.0, data.1);
    info!("Segement static data: {:x} - {:x}", static_data.0, static_data.1);

    let space = KernelAddressSpace::new(
        text,
        read_only_data,
        data,
        static_data,
        trap_code
    );

    for (segment, map) in space.into_iter() {
        if let Map::Fixed(frame_num) = map {
            let page_num = segment.range.0;

            for i in 0..(segment.range.1 - segment.range.0) {
                page_table.fixed_map(page_num + i, frame_num + i, segment.flag);
            }
        } else {
            for page_num in segment.range.0..segment.range.1 {
                page_table.insert(page_num, segment.flag);
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
    pub const END: usize = 0x88_000_000;
}
