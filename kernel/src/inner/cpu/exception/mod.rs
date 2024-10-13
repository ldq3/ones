pub mod sync_exception;
pub mod async_exception;

// use log::error;
pub fn init() {
    HandlerRv::init();
    enable_timer_interrupt();
}
    
// fn into_user() {
    // let mut sstatus = sstatus::read();
    // sstatus.set_spp(SPP::User);

    // let mut cx = Context {
        // x: [0; 32],
        // sstatus,
        // sepc: 0, // FIXME: the sepc should be the first instruction of user app
    // };

    // cx.set_sp(0);

    // Self::expt_ret(0);
// }

pub trait Context {
    fn set_sp(&mut self, sp: usize);

    fn inc_epc(&mut self, n: usize);

    fn set_ret(&mut self, ret: usize);

    fn fn_args(&self) -> [usize; 3];

    fn syscall_id(&self) -> usize;
}

pub trait KernelContext {
    
}

pub trait Handler {
    type KernelContext: KernelContext;

    fn init();
    fn distribute();
    fn get_kernel_context() -> Self::KernelContext;
}

pub use crate::inner::arch_ins::cpu::exception::*;
