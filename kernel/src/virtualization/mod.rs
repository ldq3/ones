pub mod cpu;
pub mod memory;

pub trait Virtualization {
    fn init();
}

pub struct Handler;

impl Virtualization for Handler {
    fn init() {
        use memory::Memory;
        memory::Handler::init();

        use cpu::CentralProcessUnit;
        cpu::Handler::init();
    }
}
