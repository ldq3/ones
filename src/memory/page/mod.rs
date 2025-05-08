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
use entry::Entry;
use crate::memory::{ Flag, page::frame::Frame };

#[derive(Clone, Copy)]
pub enum Map {
    Fixed(usize),
    Random,
}

pub trait Lib: Dependence {
    /**
    unsafe fn unmap(table: &mut Table, page_num: usize) {
        let entry = Self::get_mut(table, page_num);

        Self::set_flag(entry, Flag::empty());
    }
    */
    fn unmap(table: &mut Table, page_num: usize);
    /**
    将页映射到给定的页框

    # 安全性
    root
    frame_num
    page_flag

    fn map(table: &mut Table, page_num: usize, page_flag: Flag) {
        let index = Self::index(page_num);
        let root = Self::as_table(table.root());

        let mut current_entry = &mut root[index[0]];
        let mut current_table = if !Self::flag(*current_entry).is_valid() {
            let frame = Frame::new();
            *current_entry = Self::new_entry(frame.number, Flag::V);
            table.frame.push(frame);

            unsafe{ Self::table(*current_entry) }
        } else {
            unsafe{ Self::table(*current_entry) }
        };

        for i in 1..(Self::conf() - 1) {
            current_entry = &mut current_table[index[i]];
            current_table = if !Self::flag(*current_entry).is_valid() {
                let frame = Frame::new();
                *current_entry = Self::new_entry(frame.number, Flag::V);
                table.frame.push(frame);

                unsafe{ Self::table(*current_entry) }
            } else {
                unsafe{ Self::table(*current_entry) }
            }
        }

        current_entry = &mut current_table[index[Self::conf() - 1]];
        if !Self::flag(*current_entry).is_valid() {
            let frame = Frame::new();
            *current_entry = Self::new_entry(frame.number, Flag::V | page_flag);
            table.frame.push(frame);
        }
    }
    */
    fn map(table: &mut Table, page_num: usize, page_flag: Flag);

    fn map_area(table: &mut Table, page_num: (usize, usize), page_flag: Flag) {
        let (start, end) = page_num;
    
        if start > end {
            panic!("Start page number cannot be greater than end page number");
        }

        for i in 0..=(end - start) {
            Self::map(table, start + i, page_flag);
        }
    }
    /**
    unsafe fn fixed_map(table: &mut Table, page_num: usize, frame_num: usize, page_flag: Flag) { 
        let index = Self::index(page_num);
        let root = Self::as_table(table.root());

        let mut current_entry = &mut root[index[0]];
        let mut current_table = if !Self::flag(*current_entry).is_valid() {
            let frame = Frame::new();
            *current_entry = Self::new_entry(frame.number, Flag::V);
            table.frame.push(frame);
            Self::table(*current_entry)
        } else { 
            Self::table(*current_entry)
        };

        for i in 1..(Self::conf() - 1) {
            current_entry = &mut current_table[index[i]]; 
            current_table = if !Self::flag(*current_entry).is_valid() {
                let frame = Frame::new();
                *current_entry = Self::new_entry(frame.number, Flag::V);
                table.frame.push(frame);

                Self::table(*current_entry)
            } else {
                Self::table(*current_entry)
            }
        }

        current_entry = &mut current_table[index[Self::conf() - 1]];
        *current_entry = Self::new_entry(frame_num, page_flag | Flag::V);
    }
    */
    unsafe fn fixed_map(table: &mut Table, page_num: usize, frame_num: usize, page_flag: Flag); 
    /**insert page number area \[start, end]
     
    */
    unsafe fn fixed_map_area(table: &mut Table, page: (usize, usize), frame: usize, flag: Flag) {
        let (start, end) = page;

        if start > end {
            panic!("Start page number cannot be greater than end page number");
        }

        for i in 0..=(end - start) {
            Self::fixed_map(table, start + i, frame + i, flag);
        }
    }
    /**
    fn get(table: &mut Table, page_num: usize) -> (usize, Flag) {
        let index = Self::index(page_num);
        let root = Self::as_table(table.root());

        let mut current_entry = &mut root[index[0]];
        let mut current_table = unsafe{ Self::table(*current_entry) };

        for i in 1..(Self::conf() - 1) {
            current_entry = &mut current_table[index[i]];
            current_table = unsafe { Self::table(*current_entry) };
        }

        current_entry = &mut current_table[index[Self::conf() - 1]];
        (Self::frame_number(*current_entry), Self::flag(*current_entry))
    }
    */
    fn get(table: &mut Table, page_num: usize) -> (usize, Flag);
    /**
    range: the page number range
    */
    fn copy_data(&mut self, range: (usize, usize), data: &[u8]);
}

pub trait Dependence {
    fn index(page_num: usize) -> Vec<usize>;

    fn as_table(frame_number: usize) -> &'static mut [Entry] {
        use crate::memory::Address;
        use core::slice::from_raw_parts_mut;

        let address = Address::address(frame_number);
        unsafe { from_raw_parts_mut(address as *mut Entry, 512) }
    }

    fn conf() -> usize;

    /**
    fn get_mut(table: &mut Table, page_num: usize) -> &mut Entry {
        let index = Self::index(page_num);
        let table = Self::as_table(table.root.number);

        let mut current_entry = &mut table[index[0]];
        let mut current_table = unsafe{ Self::table(*current_entry) };

        for i in 1..(Self::conf() - 1) {
            current_entry = &mut current_table[index[i]];
            current_table = unsafe{ Self::table(*current_entry) };
        }

        current_entry = &mut current_table[index[Self::conf() - 1]];

        current_entry
    }
    */
    fn get_mut(table: &mut Table, page_num: usize) -> &mut Entry;
}

/**
hold frame 采用树形结构设计的问题不好确定键值对
*/
pub struct Table {
    pub root: Frame,
    pub frame: Vec<Frame>,
}

impl Table {
    pub fn new() -> Self {
        let root = Frame::new();

        Self {
            root,
            frame: Vec::new()
        }
    }
}

#[cfg(test)]
mod test {
    use alloc::vec;
    use alloc::vec::Vec;
    use crate::memory::{ Flag, Address };

    use super::{
        Dependence, Table, Lib as T, 
        entry::{ Entry, Lib as E },
        frame::Frame
    };

    struct TableLib;

    struct EntryLib;

    const FRAME_NUMBER_MASK: usize = 0b111_111_111_111_111_111_111_111_111_111_111_111_111_111_110_000_000_000;
    const FLAG_MASK: usize = 0b11_111_111_111;

    impl E for EntryLib {
        fn new(frame_num: usize, page_flag: Flag) -> Entry {
            let frame_number_bits = frame_num << FRAME_NUMBER_MASK.trailing_zeros();
            let flag_bits = (page_flag.bits as usize) << FLAG_MASK.trailing_zeros();

            Entry::from_bits(frame_number_bits | flag_bits)
        }

        fn frame_number(entry: &Entry) -> usize {
            (entry.bits() & FRAME_NUMBER_MASK) >> FRAME_NUMBER_MASK.trailing_zeros()
        }

        fn flag(entry: &Entry) -> Flag {
            let flag = (entry.bits() & FLAG_MASK) >> FLAG_MASK.trailing_zeros();

            Flag::from_bits(flag as u8).unwrap()
        }

        fn flag_set(entry: &mut Entry, page_flag: Flag) {
            let frame_number_bits = entry.bits() & FRAME_NUMBER_MASK;
            let flag_bits = (page_flag.bits as usize) << FLAG_MASK.trailing_zeros();

            entry.bits_set(frame_number_bits | flag_bits); 
        }
    }

    impl Dependence for TableLib {
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

        fn get_mut(table: &mut Table, page_num: usize) -> &mut Entry {
            let index = Self::index(page_num);
            let table = Self::as_table(table.root.number);

            let mut current_entry = &mut table[index[0]];
            let frame_number = EntryLib::frame_number(current_entry);
            let mut current_table = Self::as_table(frame_number);

            for i in 1..(Self::conf() - 1) {
                current_entry = &mut current_table[index[i]];
                let frame_number= EntryLib::frame_number(current_entry);
                current_table = Self::as_table(frame_number);
            }

            current_entry = &mut current_table[index[Self::conf() - 1]];

            current_entry
        }
    }

    impl T for TableLib { 
        fn map(table: &mut Table, page_num: usize, page_flag: Flag) {
            let index = Self::index(page_num);
            let root = Self::as_table(table.root.number);

            let mut current_entry = &mut root[index[0]];
            let mut current_table = if !EntryLib::flag(current_entry).is_valid() {
                let frame = Frame::new();
                let frame_number = frame.number;
                *current_entry = EntryLib::new(frame_number, Flag::V);
                table.frame.push(frame);

                Self::as_table(frame_number)
            } else {
                let frame_number = EntryLib::frame_number(current_entry);
                Self::as_table(frame_number)
            };

            for i in 1..(Self::conf() - 1) {
                current_entry = &mut current_table[index[i]];
                current_table = if !EntryLib::flag(current_entry).is_valid() {
                    let frame = Frame::new();
                    let frame_number = frame.number;
                    *current_entry = EntryLib::new(frame_number, Flag::V);
                    table.frame.push(frame);

                    Self::as_table(frame_number)
                } else {
                    let frame_number = EntryLib::frame_number(current_entry);
                    Self::as_table(frame_number)
                }
            }

            current_entry = &mut current_table[index[Self::conf() - 1]];
            if !EntryLib::flag(current_entry).is_valid() {
                let frame = Frame::new();
                *current_entry = EntryLib::new(frame.number, Flag::V | page_flag);
                table.frame.push(frame);
            }
        }

        fn unmap(table: &mut Table, page_num: usize) {
            let entry = Self::get_mut(table, page_num);

            EntryLib::flag_set(entry, Flag::empty());
        }

        unsafe fn fixed_map(table: &mut Table, page_num: usize, frame_num: usize, page_flag: Flag) {
            let index = Self::index(page_num); let root = Self::as_table(table.root.number);

            let mut current_entry = &mut root[index[0]];
            let mut current_table = if !EntryLib::flag(current_entry).is_valid() {
                let frame = Frame::new();
                let frame_number = frame.number;
                *current_entry = EntryLib::new(frame_number, Flag::V);
                table.frame.push(frame);
                Self::as_table(frame_number)
            } else { 
                let frame_number = EntryLib::frame_number(current_entry);
                Self::as_table(frame_number)
            };

            for i in 1..(Self::conf() - 1) {
                current_entry = &mut current_table[index[i]]; 
                current_table = if !EntryLib::flag(current_entry).is_valid() {
                    let frame = Frame::new();
                    let frame_number = frame.number;
                    *current_entry = EntryLib::new(frame_number, Flag::V);
                    table.frame.push(frame);

                    Self::as_table(frame_number)
                } else {
                    let frame_number = EntryLib::frame_number(current_entry);

                    Self::as_table(frame_number)
                }
            }

            current_entry = &mut current_table[index[Self::conf() - 1]];
            *current_entry = EntryLib::new(frame_num, page_flag | Flag::V);
        }

        fn get(table: &mut Table, page_num: usize) -> (usize, Flag) {
            let index = Self::index(page_num); let root = Self::as_table(table.root.number);

            let mut current_entry = &mut root[index[0]];
            let frame_number = EntryLib::frame_number(current_entry);
            let mut current_table = Self::as_table(frame_number);

            for i in 1..(Self::conf() - 1) {
                current_entry = &mut current_table[index[i]];
                let frame_number = EntryLib::frame_number(current_entry);
                current_table = Self::as_table(frame_number);
            }

            current_entry = &mut current_table[index[Self::conf() - 1]];
            (EntryLib::frame_number(current_entry), EntryLib::flag(current_entry))
        }

        fn copy_data(&mut self, _range: (usize, usize), _data: &[u8]) {
            todo!()
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
        unsafe{ TableLib::fixed_map(&mut table, 0, 0, Flag::V) };

        let (number, flag) = TableLib::get(&mut table, 0);

        assert_eq!(number, 0);
        assert_eq!(flag, Flag::V);
    }
}