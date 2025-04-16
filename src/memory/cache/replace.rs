use alloc::vec::Vec;

use super::{ Line, Replace };

pub struct Clock;

impl Replace for Clock {
    fn _replace(&self, line: &mut Vec<Line>, mut offset_range: core::ops::Range<usize>) -> usize { // FIXME: where mut?
        let first = if let Some(first) = offset_range.next() {
            let valid = line[first].valid;

            if valid && line[first].used == false {
                return first;
            } else {
                line[first].used = false;
            }

            first
        } else { 0 }; // FIXME

        for i in offset_range {
            let valid = line[i].valid;

            if valid && line[i].used == false {
                return i;
            } else {
                line[i].used = false;
            }
        }
  
        first
    }
}
