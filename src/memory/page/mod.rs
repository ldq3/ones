/*!
除物理空间之外，页表在内核空间之中访问

# 结构体
Address

TableEntry

Table

# 映射方式
先：
- page number:
    - fixed(frame_number)
    - random
- frame number
    - identical
*/
pub mod frame;
pub mod entry;

use alloc::vec::Vec;
use crate::memory::{ Flag, page::frame::Frame };

#[derive(Clone, Copy)]
pub enum Map {
    Fixed(usize),
    Random,
}

pub trait Table: Dependence {
    fn new() -> Self;

    /**
    Return the frame number of root table.
    */
    #[inline]
    fn root(&mut self) -> usize {
        use crate::memory::Address;
        Address::number(self.root_table().as_ptr() as usize)
    }

    unsafe fn unmap(&mut self, page_num: usize) {
        let root = self.root_table();
        let entry = Self::get_mut(root, page_num);

        Self::set_flag(entry, Flag::empty());
    }
    /**
    将页映射到给定的页框

    # 安全性
    root
    frame_num
    page_flag
    */
    fn map(&mut self, page_num: usize, page_flag: Flag) {
        let index = Self::index(page_num);
        let root = self.root_table();

        let mut current_entry = &mut root[index[0]];
        let mut current_table = if !Self::flag(*current_entry).is_valid() {
            let frame = Frame::new();
            *current_entry = Self::new_entry(frame.number, Flag::V);
            self.frame(frame);

            unsafe{ Self::table(*current_entry) }
        } else {
            unsafe{ Self::table(*current_entry) }
        };

        for i in 1..(Self::conf() - 1) {
            current_entry = &mut current_table[index[i]];
            current_table = if !Self::flag(*current_entry).is_valid() {
                let frame = Frame::new();
                *current_entry = Self::new_entry(frame.number, Flag::V);
                self.frame(frame);

                unsafe{ Self::table(*current_entry) }
            } else {
                unsafe{ Self::table(*current_entry) }
            }
        }

        current_entry = &mut current_table[index[Self::conf() - 1]];
        if !Self::flag(*current_entry).is_valid() {
            let frame = Frame::new();
            *current_entry = Self::new_entry(frame.number, Flag::V | page_flag);
            self.frame(frame);
        }
    }

    fn map_area(&mut self, page_num: (usize, usize), page_flag: Flag) {
        let (start, end) = page_num;
    
        if start > end {
            panic!("Start page number cannot be greater than end page number");
        }

        for i in 0..=(end - start) {
            self.map(start + i, page_flag);
        }
    }

    unsafe fn fixed_map(&mut self, page_num: usize, frame_num: usize, page_flag: Flag) {
        let index = Self::index(page_num);
        let root = self.root_table();

        let mut current_entry = &mut root[index[0]];
        let mut current_table = if !Self::flag(*current_entry).is_valid() {
            let frame = Frame::new();
            *current_entry = Self::new_entry(frame.number, Flag::V);
            self.frame(frame);

            Self::table(*current_entry)
        } else {
            Self::table(*current_entry)
        };

        for i in 1..(Self::conf() - 1) {
            current_entry = &mut current_table[index[i]];
            current_table = if !Self::flag(*current_entry).is_valid() {
                let frame = Frame::new();
                *current_entry = Self::new_entry(frame.number, Flag::V);
                self.frame(frame);

                Self::table(*current_entry)
            } else {
                Self::table(*current_entry)
            }
        }

        current_entry = &mut current_table[index[Self::conf() - 1]];
        *current_entry = Self::new_entry(frame_num, page_flag | Flag::V);
    }
    /**insert page number area \[start, end]
     
    */
    unsafe fn fixed_map_area(&mut self, page: (usize, usize), frame: usize, flag: Flag) {
        let (start, end) = page;

        if start > end {
            panic!("Start page number cannot be greater than end page number");
        }

        for i in 0..=(end - start) {
            self.fixed_map(start + i, frame + i, flag);
        }
    }

    fn get(&mut self, page_num: usize) -> (usize, Flag) {
        let index = Self::index(page_num);
        let root = self.root_table();

        let mut current_entry = &mut root[index[0]];
        let mut current_table = unsafe{ Self::table(*current_entry) };

        for i in 1..(Self::conf() - 1) {
            current_entry = &mut current_table[index[i]];
            current_table = unsafe { Self::table(*current_entry) };
        }

        current_entry = &mut current_table[index[Self::conf() - 1]];
        (Self::frame_number(*current_entry), Self::flag(*current_entry))
    }
    /**
    range: the page number range
    */
    fn copy_data(&mut self, range: (usize, usize), data: &[u8]);
}

pub trait Dependence {
    fn index(page_num: usize) -> Vec<usize>;

    fn conf() -> usize;

    unsafe fn table(entry: usize) -> &'static mut [usize] {
        use core::slice::from_raw_parts_mut;
        use crate::memory::Address;

        let data = Address::address(Self::frame_number(entry)) as *mut usize;

        from_raw_parts_mut(data, 512)
    }

    fn new_entry(frame_number: usize, flag: Flag) -> usize;

    fn get_mut(root: &mut [usize], page_num: usize) -> &mut usize {
        let index = Self::index(page_num);

        let mut current_entry = &mut root[index[0]];
        let mut current_table = unsafe{ Self::table(*current_entry) };

        for i in 1..(Self::conf() - 1) {
            current_entry = &mut current_table[index[i]];
            current_table = unsafe{ Self::table(*current_entry) };
        }

        current_entry = &mut current_table[index[Self::conf() - 1]];

        current_entry
    }

    fn frame_number(entry: usize) -> usize;

    fn flag(entry: usize) -> Flag;
    /**
    Set flag in entry.
    */
    fn set_flag(entry: &mut usize, flag: Flag);

    fn root_table(&mut self) -> &'static mut [usize];
    /**
    hold frame
    */
    fn frame(&mut self, frame: Frame);
}

/**
hold frame 采用树形结构设计的问题不好确定键值对
*/
pub struct ModelTable {
    pub root: Frame,
    pub frame: Vec<Frame>,
}

#[cfg(test)]
mod test {
    use core::slice::from_raw_parts_mut;

    use alloc::vec;
    use alloc::vec::Vec;
    use crate::memory::{ Flag, Address };

    use super::{
        Dependence, ModelTable, Table as T, 
        entry::{ ModelEntry, Entry as E },
        frame::Frame
    };

    struct Table(ModelTable);

    type Entry = ModelEntry<
    0b111_111_111_111_111_111_111_111_111_111_111_111_111_111_110_000_000_000,
    0b11_111_111_111,
    >;

    impl Dependence for Table {
        fn index(page_num: usize) -> Vec<usize> {
            let mut page_num = page_num;
            let mut index = [0usize; 3];

            index[2] = page_num & 0b111_111_111;
            for i in (0..2).rev() {        
                page_num >>= 9;
                index[i] = page_num & 0b111_111_111;
            }

            index.to_vec()
        }

        #[inline]
        fn conf() -> usize {
            3
        }

        unsafe fn table(entry: usize) -> &'static mut [usize] {
            let frame_number = Self::frame_number(entry);
            let address = Address::address(frame_number);

            from_raw_parts_mut(address as *mut usize, 512)
        }

        #[inline]
        fn flag(entry: usize) -> Flag {
            Entry::from_bits(entry).flag()
        }

        #[inline]
        fn new_entry(frame_number: usize, flag: Flag) -> usize {
            Entry::new(frame_number, flag).bits()
        }

        #[inline]
        fn frame_number(entry: usize) -> usize {
            Entry::from_bits(entry).frame_number()
        }

        fn set_flag(entry: &mut usize, flag: Flag) {
            let mut wrapper = Entry::from_bits(*entry);
            wrapper.set_flag(flag);

            *entry = wrapper.bits();
        }

        fn root_table(&mut self) -> &'static mut [usize] {
            let number = self.0.root.number;
            let address = Address::address(number);

            unsafe {
                from_raw_parts_mut(address as *mut usize, 512)
            }
        }

        #[inline]
        fn frame(&mut self, frame: Frame) {
            self.0.frame.push(frame);
        }
    }

    impl T for Table {
        fn new() -> Self {
            let root = Frame::new();
            let inner = ModelTable {
                root,
                frame: Vec::new()
            };

            Self(inner)
        }

        fn copy_data(&mut self, _range: (usize, usize), _data: &[u8]) {
            
        }
    }

    #[test]
    fn map() {
        let mut frame = vec![0usize; 5 * 512];

        let head = (&mut frame[511]) as *mut _ as usize;
        let head = Address::number(head);
        let tail = head + 2;
        Frame::init(head, tail);

        let mut table = Table::new();
        unsafe{ table.fixed_map(0, 0, Flag::V) };

        let (number, flag) = table.get(0);

        assert_eq!(number, 0);
        assert_eq!(flag, Flag::V);
    }
}