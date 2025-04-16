use alloc::{
    vec::Vec,
    boxed::Box
};

use crate::peripheral as device;
use super::{
    Get,
    Map, Replace, Line
};

// pub struct Through;

pub struct Back;

impl Get for Back {
    fn get_ref<'a>(&self, map: &Box<dyn Map>, replace: &Box<dyn Replace>, disk: &mut Box<dyn device::Block>, line: &'a mut Vec<Line>, address: usize) -> &'a Vec<u8> {
        if let Ok(offset) = map._search(&line, address) {
            &line[offset].data
        } else {
            let offset_range = map._cache_line_range(&line, address);
            let offset = replace._replace(line, offset_range);
            map._update_tag(line, offset, address);

            let line = &mut line[offset];
            if line.valid && line.dirty {
                disk.write(address, &line.data[..]);
            }
            disk.read(address, &mut line.data[..]);

            &line.data
        }
    }

    fn get_mut<'a>(&self, map: &Box<dyn Map>, replace: &Box<dyn Replace>, disk: &mut Box<dyn device::Block>, line: &'a mut Vec<Line>, address: usize) -> &'a mut Vec<u8> {
        if let Ok(offset) = map._search(&line, address) {
            &mut line[offset].data
        } else {
            let offset_range = map._cache_line_range(&line, address);
            let offset = replace._replace(line, offset_range);
            map._update_tag(line, offset, address);

            let line = &mut line[offset];
            if line.valid && line.dirty {
                disk.write(address, &line.data[..]);
            }
            line.dirty = true;
            disk.read(address, &mut line.data[..]);

            &mut line.data
        }
    }
}
