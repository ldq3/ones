pub mod sync_exception;
pub mod async_exception;

mod arch;

pub trait KernelContext {
    fn get() -> Self;
}

pub trait HandlerTrait {
    fn init();
    fn distribute();
}

pub use arch::riscv64::*;