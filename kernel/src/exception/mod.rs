pub mod syscall;
pub mod sync_exception;
pub mod async_exception;

// use log::error;

pub trait Context {
    fn set_sp(&mut self, sp: usize);

    fn inc_epc(&mut self, n: usize);

    fn set_ret(&mut self, ret: usize);

    fn fn_args(&self) -> [usize; 3];

    fn syscall_id(&self) -> usize;
}

pub trait Handler<T: Context> {
    fn init();

    fn into_user();

    fn hanle_exp(); 

    fn distribute(cx: &mut T) -> &mut T;
    
    fn expt_ret(cx_addr: usize);
}