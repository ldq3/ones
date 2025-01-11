pub mod sync_exception;
pub mod async_exception;
mod arch;

use arch::riscv64 as arch_ins;

// use log::error;
pub fn init() {
    arch_ins::Handler::init();
    arch_ins::enable_timer_interrupt();
}

pub trait KernelContext {
    
}

pub trait Handler {
    type KernelContext: KernelContext;

    fn init();
    fn distribute();
    fn get_kernel_context() -> Self::KernelContext;
}
