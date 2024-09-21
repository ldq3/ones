pub mod syscall;
pub mod sync_exception;
pub mod async_exception;

// use log::error;
pub fn init() {
    Handler::init();
    enable_timer_interrupt();
}

pub trait ContextTrait {
    fn set_sp(&mut self, sp: usize);

    fn inc_epc(&mut self, n: usize);

    fn set_ret(&mut self, ret: usize);

    fn fn_args(&self) -> [usize; 3];

    fn syscall_id(&self) -> usize;
}

pub trait HandlerTrait<T: ContextTrait> {
    fn init();

    fn into_user();

    fn hanle_exp(); 

    fn distribute(cx: &mut T) -> &mut T;
    
    fn expt_ret(cx_addr: usize);
}

pub use super::arch_ins::exception::*;