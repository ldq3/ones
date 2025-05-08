/*!
# 内核地址空间
2^64 B

栈的增长方向是由高地址向低地址增长

高 -> 低：
- intervene text
- intervene stack（相邻的 intervenne stack 之间存在一个保护页面）

低 -> 高：
- text
- read only data
- data
- static data
- align data

# 用户地址空间
在低地址空间中，在放置完应用 ELF 的所有段之后，会预留 4KiB 的空间作为保护页，得到地址 ustack_base ，这部分实现可以参考创建应用地址空间的 MemorySet::from_elf ， ustack_base 即为其第二个返回值。接下来从 ustack_base 开始按照 TID 从小到大的顺序向高地址放置线程的用户栈，两两之间预留一个保护页放置栈溢出。

在高地址空间中，最高的虚拟页仍然作为跳板页，跳板页中放置的是只读的代码，因此线程之间可以共享。然而，每个线程需要有自己的 Trap 上下文，于是我们在跳板页的下面向低地址按照 TID 从小到大的顺序放置线程的 Trap 上下文。也就是说，只要知道线程的 TID ，我们就可以计算出线程在所属进程地址空间内的用户栈和 Trap 上下文的位置
*/
use alloc::vec::Vec;

use crate::{
    runtime::Segment,
    memory::{ page, Flag },
};

pub trait Lib {
    fn from_elf(elf: &[u8]) -> Self;
    fn clone(&self) -> Self;
    fn new_kernel() -> Self;
    fn kernel_segement(
        mmio: &[(usize, usize)],
        text: (usize, usize),
        read_only_data: (usize, usize),
        data: (usize, usize),
        static_data: (usize, usize),
        frame: (usize, usize),
    ) -> Vec<(Segment, page::Map)> {
        let mut segement = Vec::new();
        for range in mmio {
            segement.push(
                (Segment { range: *range, growth: true, flag: Flag::R | Flag::W }, page::Map::Fixed(range.0))
            );
        }

        segement.push(
            (Segment { range: text, growth: true, flag: Flag::R | Flag::X }, page::Map::Fixed(text.0))
        );
        segement.push(
            (Segment { range: read_only_data, growth: true, flag: Flag::R }, page::Map::Fixed(read_only_data.0))
        );
        segement.push(
            (Segment { range: data, growth: true, flag: Flag::R | Flag::W }, page::Map::Fixed(data.0))
        );
        segement.push(
            (Segment { range: static_data, growth: true, flag: Flag::R | Flag::W }, page::Map::Fixed(static_data.0))
        );
        segement.push(
            (Segment { range: frame, growth: true, flag: Flag::R | Flag::W }, page::Map::Fixed(frame.0))
        );

        segement
    }
    /**
    Return the page number of intervene data page by thread id.

    Return the page number of intervene context in user program page table.
    */
    #[inline]
    fn intervene_data(tid: usize) -> (usize, Flag) {
        (config::INTERVENE_TEXT - 1 - tid, Flag::W | Flag::R)
    }
    /**
    线程的用户栈
    */
    fn stack(&self, tid: usize) -> (usize, usize, Flag);
    fn new_intervene(&mut self, tid: usize) -> usize;
    /**
    返回栈底在用户空间的地址
    */
    fn new_stack(&mut self, tid: usize) -> usize;
}

pub struct AddressSpace {
    /// Address of program entry.
    pub entry: usize,
    pub segement: Vec<Segment>,
    /// Page number of program end.
    pub stack_base: usize,
}

impl AddressSpace {
    pub fn from_elf(elf: &[u8]) -> Self {
        // map program headers of elf, with U flag
        let elf = xmas_elf::ElfFile::new(elf).unwrap();
        let magic = elf.header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");

        let entry = elf.header.pt2.entry_point() as usize;
        let mut stack_base = 0;

        let ph_count = elf.header.pt2.ph_count();
        // let mut page_table = page::Table::new();  

        let mut segement = Vec::new();
        for i in 0..ph_count {
            let program_header = elf.program_header(i).unwrap();
            if program_header.get_type().unwrap() == xmas_elf::program::Type::Load {
                let start = program_header.virtual_addr() as usize;
                let size = program_header.mem_size() as usize;
                let end = start + size - 1;
   
                use crate::memory::Address;
                let range = (Address::number(start), Address::number(end));

                let ph_flags = program_header.flags();
                let mut flag = Flag::U;
                if ph_flags.is_read() { flag |= Flag::R; }
                if ph_flags.is_write() { flag |= Flag::W; }
                if ph_flags.is_execute() { flag |= Flag::X; }

                segement.push(Segment { range, growth: true, flag });

                // page_table.map_area(range, flag);

                // page_table.copy_data(range, &elf.input[program_header.offset() as usize..(program_header.offset() + program_header.file_size()) as usize]);

                stack_base = range.1;
            }
        } 

        // unsafe { page_table.fixed_map(config::INTERVENE_TEXT, itext, Flag::X | Flag::R) };
    
        Self {
            entry,
            segement,
            stack_base,
        }
    }
}

pub mod config {
    // 4 GB = 4 * 2^30 B
    // 4 KB = 4 * 2^10 B

    /**
    Trap 相关 page

    单位：页（page）
    */
    pub const INTERVENE_TEXT: usize = (1 << 52) - 1;
}
