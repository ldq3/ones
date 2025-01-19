pub mod frame;

mod arch;

pub struct Address;

impl Address {
    pub fn guard(value: usize) -> usize {
        value & config::ADDRESS_MASK
    }

    pub fn page_num(address: usize) -> usize {
        address >> config::OFFSET_WIDTH
    }

    pub fn offset(address: usize) -> usize {
        address & config::OFFSET_MASK
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Page(usize);

impl From<usize> for Page { // FIXME
    fn from(value: usize) -> Self {
        Self(value & config::NUMBER_MASK)
    }
}

impl Page {
    fn new(frame_num: usize, page_flag: PageFlag) -> Self {
        Self((frame_num << 10) | page_flag.bits as usize)
    }

    fn frame_number(&self) -> usize {
        self.0 >> 10 & ((1usize << 44) - 1)
    }

    fn address(&self) -> usize {
        self.frame_number() << config::OFFSET_WIDTH
    }

    fn flag(&self) -> PageFlag {
        PageFlag::from_bits(self.0 as u8).unwrap()
    }

    fn set_flag(&mut self, page_flag: PageFlag) {
        self.0 = self.0 & page_flag.bits as usize
    }

    /**
    get all(512) page table entrys in a frame #FIXME
    */
    fn as_mut_pte_array(&mut self) -> &'static mut [Page] {
        unsafe { core::slice::from_raw_parts_mut(self.address() as *mut Page, 512) }
    }

    fn as_pte_array(&self) -> &'static [Page] {
        unsafe { core::slice::from_raw_parts(self.address() as *const Page, 512) }
    }
}

use bitflags::*;
bitflags! {
    pub struct PageFlag: u8 {
        const V = config::VALID;
        const R = config::READ;
        const W = config::WRITE;
        const X = config::EXECUTE;
        const U = config::USER;
        const G = config::GLOBAL;
        const A = config::ACCESSED;
        const D = config::DIRTY;
    }
}

impl PageFlag {
    fn is_valid(&self) -> bool {
        (self.bits & Self::V.bits) != Self::empty().bits
    }
}

use frame::{ alloc_frame, Frame };

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
    root: Page,
    frames: Vec<Frame>,
}

impl Table {
    pub fn new() -> Result<Self, ()> {
        if let Ok(frame) = alloc_frame() {
            Ok(Table {
                root: Page::new(frame.number, PageFlag::V),
                frames: vec![frame],
            })
        } else {
            Err(())
        }
    }

    pub fn insert(&mut self, page_num: usize, page_flag: PageFlag) -> Result<(), ()> {
        let index = index(page_num);

        let mut inner_pte = &mut self.root.as_mut_pte_array()[index[0]];

        for i in 1..3 {
            if !inner_pte.flag().is_valid() {
                if let Ok(frame) = alloc_frame() {
                    *inner_pte = Page::new(frame.number, PageFlag::V);
                    self.frames.push(frame);
                };
            }

            inner_pte = &mut inner_pte.as_mut_pte_array()[index[i]];
        }

        if !inner_pte.flag().is_valid() {
            if let Ok(frame) = alloc_frame() {
                *inner_pte = Page::new(frame.number, page_flag);
                self.frames.push(frame);
            } else {
                return Err(());
            }
        }

        Ok(())
    }

    pub fn remove(&mut self, page_num: usize) -> Result<(), ()> {
        let res = self.get_mut(page_num); // FIXME: and V
        match res {
            Ok(pte) => {
                pte.set_flag(PageFlag::empty());
                Ok(())
            },
            Err(()) => Err(())
        }
    }

    fn get_mut(&mut self, page_num: usize) -> Result<&mut Page, ()> {
        let index = index(page_num);

        let mut pte = &mut self.root.as_mut_pte_array()[index[0]];
        for i in 1..3 {
            if !pte.flag().is_valid() {
                return Err(());
            }

            pte = &mut pte.as_mut_pte_array()[index[i]];
        }

        if !pte.flag().is_valid() {
            return Err(());
        }

        Ok(pte)
    }

    pub fn get(&self, page_num: usize) -> Result<(usize, PageFlag), ()> {
        let index = index(page_num);

        let mut pte = self.root.as_pte_array()[index[0]];
        for i in 1..3 {
            if !pte.flag().is_valid() {
                return Err(());
            }

            pte = pte.as_mut_pte_array()[index[i]];
        }

        if !pte.flag().is_valid() {
            return Err(());
        }

        Ok((pte.frame_number(), pte.flag()))
    }
}

fn index(page_num: usize) -> [usize; config::LEVEL] {
    let mut page_num = page_num;
    let mut index = [0usize; 3];

    index[2] = page_num & 0b111_111_111;
    for i in (0..2).rev() {        
        page_num >>= 9;
        index[i] = page_num & 0b111_111_111;
    }

    index
}

mod config {
    pub use super::arch::riscv64::*;
}

#[cfg(test)]
mod test {
    use log::{ info, debug };
    use crate::config::initialize_logger;

    use super::*;

    #[test]
    fn insert() {
        initialize_logger();

        let space = vec![0usize; 6 * 512];
        
        if let Ok(_) = frame::init(frame::Address::number(space.as_ptr() as usize) + 1, 4) {
            assert!(true);
        } else {
            assert!(false);
        }
        debug!("Frame manager is initialized at frame number: {}.", frame::Address::number(space.as_ptr() as usize) + 1);

        let mut frame_vector = Vec::new();
        if let Ok(frame) = alloc_frame() {
            let mut page = Page::new(frame.number, PageFlag::V);
            frame_vector.push(frame);
            debug!("Frame number: {}", page.frame_number());

            let mut inner_pte = &mut page.as_mut_pte_array()[0];
            for i in 1..3 {
                if !inner_pte.flag().is_valid() {
                    if let Ok(frame) = alloc_frame() {
                        debug!("Inner frame {} number: {}", i, frame.number);
                        *inner_pte = Page::new(frame.number, PageFlag::V);
                        frame_vector.push(frame);
                    };
                }

                info!("Right.");
                inner_pte = &mut inner_pte.as_mut_pte_array()[0];
            }

            assert!(true);
        }
    }

    #[test]
    fn page_table_entry() {
        initialize_logger();

        let space = vec![2usize; 3 * 512];

        if let Ok(_) = frame::init(frame::Address::number(space.as_ptr() as usize) + 1, 1) {
            assert!(true);
        } else {
            assert!(false);
        }
        debug!("Frame manager is initialized at frame number: {:b}.", frame::Address::number(space.as_ptr() as usize) + 1);

        if let Ok(frame) = alloc_frame() {
            let mut page = Page::new(frame.number, PageFlag::V);

            debug!("Frame number: {:b}, Physical Address: {:b}", page.frame_number(), page.address());
            let pte_arr = page.as_mut_pte_array();
            pte_arr[511].0 = 3;

            let pte_arr = page.as_pte_array();

            info!("Element start and end: {} and {}.", pte_arr[0].0, pte_arr[511].0);
        }

    }
}
