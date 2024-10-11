use super::{ FrameNum, PageNum };

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
    fn get_create(&mut self, page_num: Self::PageNum) -> PageTableEntry;
}