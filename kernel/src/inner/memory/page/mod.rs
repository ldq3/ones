use crate::inner::arch_ins::memory::page::{ ADDRESS_WIDTH, FLAGS, OFFSET_WIDTH };

pub const ADDRESS_MASK: usize = (1 << ADDRESS_WIDTH) - 1;
pub const OFFSET_MASK: usize = (1 << OFFSET_WIDTH) - 1;
pub const NUMBER_MASK: usize = (1 << (ADDRESS_WIDTH - OFFSET_WIDTH)) - 1;
pub struct Flags {
    pub valid: u8,
    pub read: u8,
    pub write: u8,
    pub execute: u8,
    pub user: u8,
    pub global: u8,
    pub accessed: u8,
    pub dirty: u8,
}

pub mod frame;

#[derive(Clone, Copy)]
pub struct Address(usize);

impl From<usize> for Address {
    fn from(value: usize) -> Self {
        Self(value & ADDRESS_MASK)
    }
}

impl Into<usize> for Address {
    fn into(self) -> usize {
        self.0
    }
}

impl Address {
    #[inline]
    fn page_num(&self) -> Number {
        Number((self.0 >> OFFSET_WIDTH))
    }

    #[inline]
    fn offset(&self) -> usize {
        self.0 & OFFSET_WIDTH
    }
    
    #[inline]
    fn ceil_page_num(&self) -> Number {
        let page_num_int: usize = self.page_num().into();

        if self.offset() == 0 {
            page_num_int.into()
        } else {
            (page_num_int + 1).into()
        }
    }
}

#[derive(Clone, Copy)]
pub struct Number(usize);

impl From<usize> for Number {
    fn from(value: usize) -> Self {
        Self(value & NUMBER_MASK)
    }
}

impl Into<usize> for Number {
    fn into(self) -> usize {
        self.0
    }
}

impl Number {
    fn address(&self) -> Address {
        Address((self.0 << OFFSET_WIDTH))
    }
}

use alloc::vec;
use alloc::vec::Vec;

use frame::Frame;

use bitflags::*;
bitflags! {
    pub struct TableEntryFlag: u8 {
        const V = FLAGS.valid;
        const R = FLAGS.read;
        const W = FLAGS.write;
        const X = FLAGS.execute;
        const U = FLAGS.user;
        const G = FLAGS.global;
        const A = FLAGS.accessed;
        const D = FLAGS.dirty;
    }
}

impl TableEntryFlag {
    fn readable(&self) -> bool {
        self.contains(TableEntryFlag::R)
    }
    fn writable(&self) -> bool {
        self.contains(TableEntryFlag::W)
    }
    fn executable(&self) -> bool {
        self.contains(TableEntryFlag::X)
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TableEntry {
    pub bits: usize
}

impl TableEntry {
    fn new(frame_num: frame::Number, flags: TableEntryFlag) -> Self {
        let frame_num_int: usize = frame_num.into();

        TableEntry {
            bits: frame_num_int << 10 | flags.bits() as usize,
        }
    }

    fn empty() -> Self {
        TableEntry {
            bits: 0
        }
    }

    fn flags(&self) -> TableEntryFlag {
        TableEntryFlag::from_bits(self.bits as u8).unwrap()
    }

    fn is_valid(&self) -> bool {
        true // (self.flags() & FlagsRv::V) != FlagsRv::empty()
    }

    fn frame_num(&self) -> frame::Number {
        (self.bits >> 10 & ((1usize << 44) - 1)).into()
    }
}

/**
# Entry
a page table entry contains page number and flags

a page table entry should be like this for hardware reason:

# Frame
only page table can hold the ownership of frames

# Page Fault
- page exit: add
- page unexit: remove, modify, get

the frames hold the ownership of Frame
*/
pub struct Table {
    root: frame::Number,
    frames: Vec<Frame>,
}

// impl PageTableEntryTrait<FrameRV, PTEFlags> for PageTableEntry {
    // fn new(frame: FrameRV, flags: PTEFlags) -> Self {
        // PageTableEntry {
            // bits: frame.ppn << 10 | flags.bits as usize,
        // }
    // }
// }

impl Table {
    fn new() -> Result<Self, ()> {
        let res_frame = Frame::new();
        
        match res_frame {
            Ok(frame) => {
                let page_table = Table {
                    root: frame.number,
                    frames: vec![frame],
                };

                Ok(page_table)
            },
            Err(_) => Err(()),
        }

    }
    
    fn insert(&mut self, page_num: Number, pte: TableEntry) -> Result<(), ()> {
        let index = index(page_num);
        let mut existed = true;

        let mut pte_before = &mut get_ptes_in_frame(self.root)[index[0]];    
        for i in 1..3 {
            if !pte_before.is_valid() {
                existed = false;

                let frame = Frame::new().unwrap();
                *pte_before = TableEntry::new(frame.number, TableEntryFlag::from_bits(0).unwrap()); //      F::from_bits(0)); // FlagsRv::V);

                self.frames.push(frame);
            }

            pte_before = &mut get_ptes_in_frame(pte_before.frame_num())[index[i]];
        }

        if !pte_before.is_valid() {
            existed = false;

            let frame = Frame::new().unwrap();
            *pte_before = TableEntry::new(frame.number, TableEntryFlag::from_bits(0).unwrap()); //       F::from_bits(0)); // FlagsRv::V);

            self.frames.push(frame);
        }

        if existed == true {
            Err(())
        } else {
            *pte_before = pte;

            Ok(())
        }
    }

    fn remove(&mut self, page_num: Number) -> Result<(), ()> {
        let res = get_mut_ref(self, page_num);
        match res {
            Ok(pte) => {
                *pte = TableEntry::empty();
                Ok(())
            },
            Err(()) => Err(())
        }
    }

    fn replace(&mut self, page_num: Number, pte: TableEntry) -> Result<(), ()> {
        let res = get_mut_ref(self, page_num);
        match res {
            Ok(pte_before) => {
                *pte_before = pte;
                Ok(())
            },
            Err(()) => Err(())
        }
    }

    fn get(&self, page_num: Number) -> Result<TableEntry, ()> {
        let index = index(page_num);

        let mut pte = get_ptes_in_frame(self.root)[index[0]];
        for i in 1..3 {
            if !pte.is_valid() {
                return Err(());
            }

            pte = get_ptes_in_frame(pte.frame_num())[index[i]];
        }

        if !pte.is_valid() {
            return Err(());
        }

        Ok(pte)
    }

    fn get_create(&mut self, page_num: Number) -> TableEntry {
        let index = index(page_num);

        let mut pte = &mut get_ptes_in_frame(self.root)[index[0]];    
        for i in 1..3 {
            if !pte.is_valid() {
                let frame = Frame::new().unwrap();
                *pte = TableEntry::new(frame.number, TableEntryFlag::from_bits(0).unwrap()); // FlagsRv::V);

                self.frames.push(frame);
            }
    
            pte = &mut get_ptes_in_frame(pte.frame_num())[index[i]];
        }

        if !pte.is_valid() {
            let frame = Frame::new().unwrap();
            *pte = TableEntry::new(frame.number, TableEntryFlag::from_bits(0).unwrap()); // FlagsRv::V);

            self.frames.push(frame);
        }

        *pte
    }
}

/**
get all(512) page table entrys in a frame
*/
fn get_ptes_in_frame(frame_num: frame::Number) -> &'static mut [TableEntry] {
    let physical_address: usize = frame_num.address().into();
    unsafe { core::slice::from_raw_parts_mut(physical_address as *mut TableEntry, 512) }
}

fn index(page_num: Number) -> [usize; 3] {
    let mut index = [0usize; 3];

    let mut page_num_int: usize = page_num.into();
    index[2] = page_num_int & 0b111_111_111;
    for i in (0..2).rev() {        
        page_num_int >>= 9;
        index[i] = page_num_int & 0b111_111_111;
    }

    index
}

fn get_mut_ref(page_table: &mut Table, page_num: Number) -> Result<&mut TableEntry, ()> {
    let index = index(page_num);

    let mut pte = &mut get_ptes_in_frame(page_table.root)[index[0]];
    for i in 1..3 {
        if !pte.is_valid() {
            return Err(());
        }

        pte = &mut get_ptes_in_frame(pte.frame_num())[index[i]];
    }

    if !pte.is_valid() {
        return Err(());
    }

    Ok(pte)
}

fn get_mut_ref_create(page_table: &mut Table, page_num: Number) -> &mut TableEntry {
    let index = index(page_num);

    let mut pte = &mut get_ptes_in_frame(page_table.root)[index[0]];    
    for i in 1..3 {
        if !pte.is_valid() {
            let frame = Frame::new().unwrap();
            *pte = TableEntry::new(frame.number, TableEntryFlag::from_bits(0).unwrap()); // FlagsRv::V);

            page_table.frames.push(frame);
        }
    
        pte = &mut get_ptes_in_frame(pte.frame_num())[index[i]];
    }

    if !pte.is_valid() {
        let frame = Frame::new().unwrap();
        *pte = TableEntry::new(frame.number, TableEntryFlag::from_bits(0).unwrap()); // FlagsRv::V);

        page_table.frames.push(frame);
    }

    pte
}