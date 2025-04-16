use alloc::vec::Vec;

use super::{ Map, Line };

pub struct Direct;

impl Map for Direct {
    fn _cache_line_range(&self, line: &Vec<Line>, _address: usize) -> core::ops::Range<usize> {
        0..line.len()
    }

    fn _search(&self, line: &Vec<Line>, address: usize) -> Result<usize, ()> {
        for i in 0..line.len() {
            let valid = line[i].valid;
            let tag = line[i].tag;

            if valid
            && tag == address {
                return Ok(i);
            }
        }

        Err(())
    }

    fn _update_tag(&self, line: &mut Vec<Line>, offset: usize, address: usize) {
        line[offset].tag = address; 
    }
}
