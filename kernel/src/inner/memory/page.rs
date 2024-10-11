pub trait FrameNum: Into<usize> + From<usize> {
    type PhysicalAddress: PhysicalAddress;

    fn physical_address(&self) -> Self::PhysicalAddress;
}

pub trait PhysicalAddress: From<usize> + Into<usize> {
    type F: FrameNum;

    fn frame_num(&self) -> Self::F;

    fn offset(&self) -> usize;

    #[inline]
    fn ceil_frame_num(&self) -> Self::F {
        let frame_num_int: usize = self.frame_num().into();
        
        if self.offset() == 0 {
            frame_num_int.into()
        } else {
            (frame_num_int + 1).into()
        }
    }
}

use core::mem::needs_drop;
// #FIXME: !Copy
pub trait Frame: Sized {
    fn new() -> Result<Self, ()>;
}

pub trait FrameManager {
    fn new(start: usize, end: usize) -> Self;
    fn init(&mut self);
    fn alloc() -> usize;
    fn dealloc(frame: usize);
}

pub trait PageNum: Into<usize> + From<usize> {}

pub trait VirtualAddress: From<usize> + Into<usize> {
    type P: PageNum;

    fn page_num(&self) -> Self::P;

    fn offset(&self) -> usize;

    #[inline]
    fn ceil_page_num(&self) -> Self::P {
        let page_num_int = self.page_num().into();

        if self.offset() == 0 {
            page_num_int.into()
        } else {
            (page_num_int + 1).into()
        }
    }
}

pub trait Flags {
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
    fn executable(&self) -> bool;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PageTableEntry {
    pub bits: usize
}

pub trait PageTableEntryTrait {
    type FrameNum: FrameNum;
    type Flags: Flags;
    
    fn empty() -> PageTableEntry {
        PageTableEntry { bits: 0 }
    }

    fn new(frame_num: Self::FrameNum, flags: Self::Flags) -> Self;
    fn frame_num(&self) -> Self::FrameNum;
    fn flags(&self) -> Self::Flags;
    fn is_valid(&self) -> bool;
}

/**
# Entry
a page table entry contains page number and flags

a page table entry should be like this for hardware reason:
```rust
#[repr(C)]
struct Entry {
    bits: usize
}
```

# Frame
only page table can hold the ownership of frames

# Page Fault
- page exit: add
- page unexit: remove, modify, get
*/
pub trait PageTable: Sized {
    type PageNum: PageNum;

    fn new() -> Result<Self, ()>;
    fn insert(&mut self, page_num: Self::PageNum, pte: PageTableEntry) -> Result<(), ()>;
    fn remove(&mut self, page_num: Self::PageNum) -> Result<(), ()>;
    fn replace(&mut self, page_num: Self::PageNum, pte: PageTableEntry) -> Result<(), ()>;
    fn get(&self, page_num: Self::PageNum) -> Result<PageTableEntry, ()>;
}
