use crate::file_system::Inode;
use super::Flag;

pub struct File {
    // size: u32,
    _flag: Flag,
    offset: usize,
    inode: Arc<Inode>,
}

use alloc::sync::Arc;
impl File {
    pub fn new(flag: Flag, inode: Arc<Inode>) -> Self {
        Self {
            _flag: flag,
            offset: 0,
            inode
        }
    }

    pub fn read(&mut self, mut buf: UserBuffer) -> usize {
        let mut total_read_size = 0usize;
        for slice in buf.buffers.iter_mut() {
            let read_size = self.inode.read_at(self.offset, *slice);
            if read_size == 0 {
                break;
            }
            self.offset += read_size;
            total_read_size += read_size;
        }
        total_read_size
    }
    
    pub fn write(&mut self, buf: UserBuffer) -> usize {
        let mut total_write_size = 0usize;
        for slice in buf.buffers.iter() {
            let write_size = self.inode.write_at(self.offset, *slice);
            assert_eq!(write_size, slice.len());
            self.offset += write_size;
            total_write_size += write_size;
        }
        total_write_size
    }

    pub fn read_all(&mut self) -> Vec<u8> {
        let mut buffer = [0u8; 512];
        let mut v: Vec<u8> = Vec::new();
        loop {
            let len = self.inode.read_at(self.offset, &mut buffer);
            if len == 0 {
                break;
            }
            self.offset += len;
            v.extend_from_slice(&buffer[..len]);
        }
        v
    }
}

pub struct UserBuffer {
    pub buffers: Vec<&'static mut [u8]>,
}

use alloc::vec::Vec;
impl UserBuffer {
    pub fn new(buffers: Vec<&'static mut [u8]>) -> Self {
        Self { buffers }
    }
    pub fn len(&self) -> usize {
        let mut total: usize = 0;
        for b in self.buffers.iter() {
            total += b.len();
        }
        total
    }
}

impl IntoIterator for UserBuffer {
    type Item = *mut u8;
    type IntoIter = UserBufferIterator;
    fn into_iter(self) -> Self::IntoIter {
        UserBufferIterator {
            buffers: self.buffers,
            current_buffer: 0,
            current_idx: 0,
        }
    }
}

pub struct UserBufferIterator {
    buffers: Vec<&'static mut [u8]>,
    current_buffer: usize,
    current_idx: usize,
}

impl Iterator for UserBufferIterator {
    type Item = *mut u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_buffer >= self.buffers.len() {
            None
        } else {
            let r = &mut self.buffers[self.current_buffer][self.current_idx] as *mut _;
            if self.current_idx + 1 == self.buffers[self.current_buffer].len() {
                self.current_idx = 0;
                self.current_buffer += 1;
            } else {
                self.current_idx += 1;
            }
            Some(r)
        }
    }
}