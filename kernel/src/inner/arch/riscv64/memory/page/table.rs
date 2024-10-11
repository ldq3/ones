use alloc::vec;
use alloc::vec::Vec;

use super::{FrameRV, PageNumRv39, FrameNumRv, PhysicalAddressRv64};
use crate::inner::memory::page::{FrameNum, Frame, Flags, PageTableEntry, PageTableEntryTrait};

use bitflags::*;
bitflags! {
    pub struct FlagsRv: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}

impl Flags for FlagsRv {
    fn readable(&self) -> bool {
        self.contains(FlagsRv::R)
    }
    fn writable(&self) -> bool {
        self.contains(FlagsRv::W)
    }
    fn executable(&self) -> bool {
        self.contains(FlagsRv::X)
    }
}

impl PageTableEntryTrait for PageTableEntry {
    type Flags = FlagsRv;
    type FrameNum = FrameNumRv;

    fn new(frame_num: Self::FrameNum, flags: Self::Flags) -> Self {
        let frame_num_int: usize = frame_num.into();

        PageTableEntry {
            bits: frame_num_int << 10 | flags.bits as usize,
        }
    }

    fn flags(&self) -> Self::Flags {
        FlagsRv::from_bits(self.bits as u8).unwrap()
    }

    fn is_valid(&self) -> bool {
        (self.flags() & FlagsRv::V) != FlagsRv::empty()
    }

    fn frame_num(&self) -> Self::FrameNum {
        (self.bits >> 10 & ((1usize << 44) - 1)).into()
    }
}

/**
the frames hold the ownership of Frame
*/
pub struct PageTableRv39 {
    root: FrameNumRv,
    frames: Vec<FrameRV>,
}

// impl PageTableEntryTrait<FrameRV, PTEFlags> for PageTableEntry {
    // fn new(frame: FrameRV, flags: PTEFlags) -> Self {
        // PageTableEntry {
            // bits: frame.ppn << 10 | flags.bits as usize,
        // }
    // }
// }

use crate::inner::memory::page::PageTable;

impl PageTable for PageTableRv39 {
    type PageNum = PageNumRv39;
    
    fn new() -> Result<Self, ()> {
        let res_frame = FrameRV::new();
        
        match res_frame {
            Ok(frame) => {
                let page_table = PageTableRv39 {
                    root: frame.ppn,
                    frames: vec![frame],
                };

                Ok(page_table)
            },
            Err(_) => Err(()),
        }

    }
    
    fn insert(&mut self, page_num: Self::PageNum, pte: PageTableEntry) -> Result<(), ()> {
        Err(())
    }

    fn remove(&mut self, page_num: Self::PageNum) -> Result<(), ()> {
        let res_get = self.get(page_num);     

        let mut pte = res_get.unwrap();
    
        pte = PageTableEntry::empty();

        Err(())
    }

    fn replace(&mut self, page_num: Self::PageNum, pte: PageTableEntry) -> Result<(), ()> {
        Err(())
    }

    fn get(&self, page_num: Self::PageNum) -> Result<PageTableEntry, ()> {
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

    fn get_create(&mut self, page_num: Self::PageNum) -> PageTableEntry {
        let index = index(page_num);

        let mut pte = &mut get_ptes_in_frame(self.root)[index[0]];    
        for i in 1..3 {
            if !pte.is_valid() {
                let frame = FrameRV::new().unwrap();
                *pte = PageTableEntry::new(frame.ppn, FlagsRv::V);
                self.frames.push(frame);
            }
    
            pte = &mut get_ptes_in_frame(pte.frame_num())[index[i]];
        }

        if !pte.is_valid() {
            let frame = FrameRV::new().unwrap();
            *pte = PageTableEntry::new(frame.ppn, FlagsRv::V);
            self.frames.push(frame);
        }

        *pte
    }
}

/**
get all(512) page table entrys in a frame
*/
fn get_ptes_in_frame(frame_num: FrameNumRv) -> &'static mut [PageTableEntry] {
    let physical_address = frame_num.physical_address();
    unsafe { core::slice::from_raw_parts_mut(physical_address.0 as *mut PageTableEntry, 512) }
}

fn index(page_num: PageNumRv39) -> [usize; 3] {
    let mut index = [0usize; 3];

    let mut page_num_int: usize = page_num.into();
    index[2] = page_num_int & 0b111_111_111;
    for i in (0..2).rev() {        
        page_num_int >>= 9;
        index[i] = page_num_int & 0b111_111_111;
    }

    index
}

fn get_mut_ref(page_table: &mut PageTableRv39, page_num: PageNumRv39) -> Result<&mut PageTableEntry, ()> {
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

fn get_mut_ref_create(page_table: &mut PageTableRv39, page_num: PageNumRv39) -> &mut PageTableEntry {
    let index = index(page_num);

    let mut pte = &mut get_ptes_in_frame(page_table.root)[index[0]];    
    for i in 1..3 {
        if !pte.is_valid() {
            let frame = FrameRV::new().unwrap();
            *pte = PageTableEntry::new(frame.ppn, FlagsRv::V);
            page_table.frames.push(frame);
        }
    
        pte = &mut get_ptes_in_frame(pte.frame_num())[index[i]];
    }

    if !pte.is_valid() {
        let frame = FrameRV::new().unwrap();
        *pte = PageTableEntry::new(frame.ppn, FlagsRv::V);
        page_table.frames.push(frame);
    }

    pte
}
