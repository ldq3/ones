pub mod sync_exception;
pub mod async_exception;

use crate::inner::arch_ins::cpu::exception;

// use log::error;
pub fn init() {
    exception::Handler::init();
    exception::enable_timer_interrupt();
}

pub trait KernelContext {
    
}

pub trait Handler {
    type KernelContext: KernelContext;

    fn init();
    fn distribute();
    fn get_kernel_context() -> Self::KernelContext;
}
