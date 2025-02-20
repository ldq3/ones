/*！
ttext 节的位置不能动

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
use ones::virtualization::memory::{ page::{Address as _, Table}, Memory };

pub mod page;

pub struct Handler;

impl Memory for Handler {
    fn init() {
        use ones::runtime::KernelAddressSpace;

        use crate::virtualization::memory::page::frame::PhysicalAddress;

        #[allow(unused)]
        extern "C" {
            fn stext();
            fn ttext();
            fn etext();

            fn srodata();
            fn erodata();

            fn sdata();
            fn edata();

            fn kernel_stack();
            // sbss
            fn ebss();
            fn ekernel();
        }

        let start = PhysicalAddress::number(ekernel as usize);
        let length = PhysicalAddress::number(config::END) - start;

        use ones::virtualization::memory::page::frame;
        frame::init(start, length);
        info!("Frame manager is initialized at: {:x}, length: {:x}", start, length);

        let text = (PhysicalAddress::number(stext as usize), PhysicalAddress::number(etext as usize));
        let trap_text = PhysicalAddress::number(ttext as usize);
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

        Self::init_page(space);

        Self::enable_page();
    }

    #[inline]
    fn enable_page() {
        use riscv::register::satp;
        use core::arch::asm;

        unsafe {
            satp::write(1usize << 63 | page::KERNEL_TABLE.lock().root.1.number);
            asm!("sfence.vma");
        }
    }

    #[inline]
    fn page_to_frame(page_num: usize) -> usize {
        let mut kernel_page_table = page::KERNEL_TABLE.lock();

        let (frame_num, _) = kernel_page_table.get(page_num).unwrap();

        frame_num
    }
    
    #[inline]
    fn map(page_num: usize, flag: ones::virtualization::memory::Flag) {
        let mut kernel_page_table = page::KERNEL_TABLE.lock();

        kernel_page_table.insert(page_num, flag);
    }

    #[inline]
    fn fixed_map(page_num: usize, frame_num: usize, flag: ones::virtualization::memory::Flag) {
        let mut kernel_page_table = page::KERNEL_TABLE.lock();

        kernel_page_table.fixed_map(page_num, frame_num, flag);
    }
}

#[allow(unused)]
mod test {
    use log::info;

    pub fn kernel_page_table(trap_text: usize) {
        use ones::virtualization::memory::page::Table as _;
        use crate::virtualization::memory::page::KERNEL_TABLE;

        let mut page_table = KERNEL_TABLE.lock();

        use ones::virtualization::memory::config::TRAP_TEXT;
        if let Ok((frame_num, _)) = page_table.get(TRAP_TEXT) {
            assert_eq!(frame_num, trap_text);
            info!("Segement trap text is mapped successfully, VA: {:x}, PA: {:x}", TRAP_TEXT, frame_num);
        } else {
            panic!("Text segement map error.");
        }
    }
}

mod config {
    pub const END: usize = 0x88_000_000;
}
