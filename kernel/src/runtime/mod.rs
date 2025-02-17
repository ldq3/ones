/*!
file entry.asm under every architecture subfolder is needed. 初始化栈
*/

pub mod lang_items;
pub mod memory;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

pub trait Runtime {
    fn init();
}

pub struct Handler;

impl Runtime for Handler {
    #[inline]
    fn init() {
        use memory::Memory;
        memory::Handler::init();
    }
}

mod config {
    
}

mod test {

}