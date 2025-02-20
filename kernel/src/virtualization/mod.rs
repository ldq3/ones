pub mod cpu;
pub mod memory;
mod process;

pub trait Virtualization {
    fn init();
}

pub struct Handler;

impl Virtualization for Handler {
    fn init() {
        use ones::virtualization::{ memory::Memory, cpu::CentralProcessUnit };
        
        memory::Handler::init();

        cpu::Handler::init();
    }
}
