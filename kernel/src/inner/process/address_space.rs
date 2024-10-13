pub mod config {
    // pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
    pub const CONTEXT_ADDR: usize = 0; // TRAMPOLINE - PAGE_SIZE;
}

use alloc::vec::Vec;

use crate::inner::memory::{
    page::table::PageTable,
    segment::Segment
};

/*
struct AddressSpace<P: PageTable> {
    segments: Vec<Segment>,
    page_table: P,
}
*/

pub enum Map {
    Identical,
    Random,
}

fn map() {

}

fn unmap() {

}