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

pub trait Memory {
    fn init();
}

pub struct Handler;

impl Memory for Handler {
    fn init() {
        use ones::virtualization::memory::{ KernelAddressSpace, page::Map };

        use crate::virtualization::memory::page::{ self, frame::PhysicalAddress, KERNEL_PAGE_TABLE };

        #[allow(unused)]
        extern "C" {
            fn stext();
            fn etext();

            fn ttest();

            fn srodata();
            fn erodata();

            fn sdata();
            fn edata();

            fn kernel_stack();
            // sbss
            fn ebss();
            fn ekernel();
        }

        let start = PhysicalAddress::ceil_number(ekernel as usize);
        let length = PhysicalAddress::number(config::END) - start;

        page::init(start, length);
        info!("Frame manager is initialized at: {:x}, length: {:x}", start, length);

        let mut page_table = KERNEL_PAGE_TABLE.lock();

        let text = (PhysicalAddress::number(stext as usize), PhysicalAddress::number(etext as usize));
        let trap_text = PhysicalAddress::number(ttest as usize);
        let read_only_data = (PhysicalAddress::number(srodata as usize), PhysicalAddress::number(erodata as usize));
        let data = (PhysicalAddress::number(sdata as usize), PhysicalAddress::number(edata as usize));
        let static_data = (PhysicalAddress::number(kernel_stack as usize), PhysicalAddress::number(ebss as usize));

        info!("Segement text: {:x} - {:x}", text.0, text.1);
        info!("Segement trap text: {:x}", trap_text);
        info!("Segement read only data: {:x} - {:x}", read_only_data.0, read_only_data.1);
        info!("Segement data: {:x} - {:x}", data.0, data.1);
        info!("Segement static data: {:x} - {:x}", static_data.0, static_data.1);

        let space = KernelAddressSpace::new(
            text,
            read_only_data,
            data,
            static_data,
            trap_text
        );

        for (segment, map) in space.into_iter() {
            if let Map::Fixed(frame_num) = map {
                let page_num = segment.range.0;

                for i in 0..(segment.range.1 - segment.range.0 + 1) {
                    page_table.fixed_map(page_num + i, frame_num + i, segment.flag);
                }
            } else {
                for page_num in segment.range.0..(segment.range.1 + 1) {
                    page_table.insert(page_num, segment.flag);
                }
            }
        }

        // test kernel page table
        {
            if let Ok((frame_num, _)) = page_table.get(text.1) {
                assert_eq!(frame_num, text.1)
            } else {
                panic!("Text segement map error.")
            }
        }

        use riscv::register::satp;
        use core::arch::asm;
        unsafe {
            satp::write(1usize << 63 | page_table.root.1.number);
            asm!("sfence.vma");
        }
    }
}

mod config {
    pub const END: usize = 0x88_000_000;
}
