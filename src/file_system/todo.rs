/*！
站在用户的角度看来，在一个进程中可以使用多种不同的标志来打开一个文件，这会影响到打开的这个文件可以用何种方式被访问

此外，在连续调用 sys_read/write 读写一个文件的时候，我们知道进程中也存在着一个文件读写的当前偏移量，它也随着文件读写的进行而被不断更新

一个进程可以访问的多个文件，所以在操作系统中需要有一个管理进程访问的多个文件的结构，这就是 文件描述符表 (File Descriptor Table) ，其中的每个 文件描述符 (File Descriptor) 代表了一个特定读写属性的I/O资源。

文件描述符 (File Descriptor) 则是一个非负整数，表示文件描述符表中一个打开的 文件描述符 所处的位置（可理解为数组下标）。进程通过文件描述符，可以在自身的文件描述符表中找到对应的文件记录信息，从而也就找到了对应的文件，并对文件进行读写。当打开（ open ）或创建（ create ） 一个文件的时候，一般情况下内核会返回给应用刚刚打开或创建的文件对应的文件描述符；而当应用想关闭（ close ）一个文件的时候，也需要向内核提供对应的文件描述符，以完成对应文件相关资源的回收操作。
*/

pub mod file;

// pub struct Handler {
//     index: BitMap,
//     data: BitMap,
// }

// impl Handler {
//     fn new() { // -> Arc<Mutex<Self>> {
//         // Arc::new(Mutex::new())
//     }

//     #[inline]
//     fn root(&self) {
        
//     }
// }

// impl FileOperation for Handler {}

use file::File;
use crate::easy_file_system::{ BlockDevice, EasyFileSystem, Inode };

// #[repr(C)]
// struct MetaData {
//     pub size: u32,
//     index_bitmap: u32,
//     index: u32,
//     data_bitmap: u32,
//     data: u32,
// }

pub struct BitMap(Vec<u8>);

impl BitMap {
    pub fn set(&mut self, order: u32, value: bool) -> Result<(), ()> {
        let byte_index = (order / 8) as usize;

        if byte_index >= self.0.len() { return Err(()); }

        let bit_index = (order % 8) as u8;

        if value {
            self.0[byte_index] |= 1 << bit_index;
        } else {
            self.0[byte_index] &= !(1 << bit_index);
        }

        Ok(())
    }

    pub fn test(&self, order: u32) -> Result<bool, ()> {
        let byte_index = (order / 8) as usize;

        if byte_index >= self.0.len() { return Err(()); }

        let bit_index = (order % 8) as u8;

        Ok((self.0[byte_index] & (1 << bit_index)) != 0)
    }

    pub fn alloc(&self) -> Result<u32, ()> {
        for i in 0..(8 * self.0.len()) {
            if let Ok(false) = self.test(i as u32) { return Ok(i as u32); }
        }

        Err(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bitmap() {
        let vector = [1u8, 0, 1].to_vec();
        let mut bitmap = BitMap(vector);

        if let Err(_) = bitmap.set(24, true) { 
            assert!(true);
        } else {
            assert!(false);
        }

        if let Err(_) = bitmap.test(24) { 
            assert!(true);
        } else {
            assert!(false);
        }

        if let Ok(true) = bitmap.test(0) {
            assert!(true);
        } else {
            assert!(false);
        }

        if let Ok(1) = bitmap.alloc() {
            assert!(true);
        } else {
            assert!(false);
        }

        if let Ok(_) = bitmap.set(0, false) {
            if let Ok(false) = bitmap.test(0) {
                assert!(true);
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }
}