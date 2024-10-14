pub mod exception;
pub mod context;
pub mod timer;

pub fn satp_from_page_table() -> usize {
    8usize << 60 // | self.root_ppn.0
}