mod arch;
mod timer;
mod exception;

pub use arch::riscv as arch_ins; // architecture instance

pub fn init() {
    exception::init();
    timer::init();  
}