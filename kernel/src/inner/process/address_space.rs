// config
pub const GLOBAL_CODE_PAGE_NUMBER: usize = 0;// usize::MAX - PAGE_SIZE + 1;
pub const GLOBAL_DATA_PAGE_NUMBER: usize = 0; // TRAMPOLINE - PAGE_SIZE;

use alloc::vec::Vec;

use crate::inner::memory::{
    page::Table,
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