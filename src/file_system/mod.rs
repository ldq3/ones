/**
Temporary outter crate.
*/

mod bitmap;
mod block_cache;
mod efs;
mod layout;
mod vfs;
pub mod file;

use alloc::sync::Arc;
use spin::Mutex;

pub const BLOCK_SZ: usize = 512;
use bitmap::Bitmap;
use block_cache::{ block_cache_sync_all, get_block_cache };
pub use crate::peripheral::Block as BlockDevice;
pub use efs::EasyFileSystem;
use layout::*;
pub use vfs::Inode;
use file::File;

pub trait Lib {
    fn init(disk: Arc<Mutex<dyn BlockDevice>>) {
        let efs = EasyFileSystem::open(disk.clone());
        let mut handler = HANDLER.lock();
        *handler = Some(EasyFileSystem::root(&efs));
    }

    fn open_file(name: &str, flag: Flag) -> Result<File, ()> {
        if flag.contains(Flag::CREATE) {
            if let Ok(inode) = Self::get(name) {
                // clear size
                inode.clear();
                Ok(File::new(flag, inode))
            } else {
                Self::create(name)
                .map(|inode| File::new(flag, inode))
            }
        } else {
            Self::get(name)
            .map(|inode| {
                if flag.contains(Flag::TRUNC) {
                    inode.clear();
                }
                File::new(flag, inode)
            })
        }
    }

    fn close_file() {

    }

    fn read() {

    }

    fn write() {

    }

    fn create(name: &str) -> Result<Arc<Inode>, ()>;
    /**
    Get an inode by nameSS
    */
    fn get(name: &str) -> Result<Arc<Inode>, ()> {
        let mutex = HANDLER.lock();
        if let Some(handler) = mutex.as_ref() {
            handler.find(name)
        } else { panic!("Initialize the file system first."); }
    }
}

use bitflags::bitflags;
bitflags! {
    pub struct Flag: u32 {
        const READ = 0;
        const WRITE = 1 << 0;
        const R_W = 1 << 1;
        const CREATE = 1 << 9;
        /// truncate，截断
        const TRUNC = 1 << 10;
    }
}

impl Flag {
    /**
    readable
    
    Didn't check validity for simplicity.
    */
    #[inline]
    pub fn read(&self) -> bool {
        if self.contains(Self::WRITE) {
            false
        } else {
            true
        }
    }

    /**
    writable
    
    Didn't check validity for simplicity.
    */
    #[inline]
    pub fn write(&self) -> bool {
        if self.is_empty() {
            false
        } else {
            true
        }
    }
}

use lazy_static::lazy_static;
lazy_static! {
    static ref HANDLER: Mutex<Option<Inode>> = Mutex::new(None);
}