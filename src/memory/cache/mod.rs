/*!
address, 字节编址
cache_line, offset
disk_line

地址结构（左高右低）：标签位（tag bit）、索引位（index bit）、行内偏移（line offset）
*/

use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;
use core::any::Any;

use crate::peripheral as device;

pub mod map;
pub mod replace;
pub mod get;

pub struct Cache {
    /// 行地址的位数
    line_bit: usize,
    line: Vec<Line>, // FIXME: 返回引用的设计，Option 的位置，尽量不要返回对枚举中数据的引用
    
    disk: Box<dyn device::Block>,

    map: Box<dyn Map>,
    replace: Box<dyn Replace>,
    get: Box<dyn Get>,
}

pub trait Map: Send + Sync + Any {
    fn _cache_line_range(&self, line: &Vec<Line>, disk_line: usize) -> core::ops::Range<usize>;

    fn _search(&self, line: &Vec<Line>, disk_line: usize) -> Result<usize, ()>;

    /**
    将字节地址（address）解析为 disk_line 和 offset
    */
    fn parse(&self, address: usize, line_bit: usize) -> (usize, usize) {
        let disk_line = address >> line_bit;

        let mask = (1 << line_bit) -1;
        let offset = address & mask;

        (disk_line, offset)
    }

    /**
    注意 cache_line 应该位于 cache_line_range(address) 内
    */
    fn _update_tag(&self, line: &mut Vec<Line>, cache_line: usize, disk_line: usize);
}

pub trait Replace: Send + Sync + Any {
    /**
    # 返回
    偏移量
    */
    fn _replace(&self, line: &mut Vec<Line>, cache_line_range: core::ops::Range<usize>) -> usize;
}

pub trait Get: Send + Sync + Any {
    fn get_ref<'a>(&self, map: &Box<dyn Map>, replace: &Box<dyn Replace>, disk: &Box<dyn device::Block>, line: &'a mut Vec<Line>, disk_line: usize) -> &'a Vec<u8>;
    
    fn get_mut<'a>(&self, map: &Box<dyn Map>, replace: &Box<dyn Replace>, disk: &Box<dyn device::Block>, line: &'a mut Vec<Line>, disk_line: usize) -> &'a mut Vec<u8>;
}

impl Cache {
    pub fn new(disk: Box<dyn device::Block>, map: Box<dyn Map>, replace: Box<dyn Replace>, get: Box<dyn Get>) -> Self {
        Self{
            line_bit: 1,
            line: vec![Line::new(); config::SIZE],

            disk,
            
            map,
            replace,
            get,
        }
    }

    pub fn read(&mut self, address: usize, number: usize) -> Vec<u8> {
        let (disk_line, offset) = self.map.parse(address, self.line_bit);

        let cache = self.get.get_ref(&self.map, &self.replace, &self.disk, &mut self.line, disk_line);

        cache[offset..number].to_vec()
    }

    pub fn write(&mut self, address: usize, data: &Vec<u8>) {
        let (disk_line, offset) = self.map.parse(address, self.line_bit);

        let cache = self.get.get_mut(&self.map, &self.replace, &self.disk, &mut self.line, disk_line);

        for i in 0..data.len() {
            cache[offset + i] = data[i]
        }
    }
}

#[derive(Clone)]
pub struct Line {
    valid: bool,
    used: bool,
    dirty: bool,

    tag: usize,
    
    data: Vec<u8>,
}

impl Line {
    fn new() -> Self {
        Self {
            valid: false,
            used: true,
            dirty: false,

            tag: 0,

            data: vec![0; 512],
        }
    }
}

mod config {
    pub const SIZE: usize = 16;
}
