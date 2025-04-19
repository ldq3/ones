/*!
initialize context
let mut sstatus = sstatus::read();
// set CPU privilege to User after trapping back
sstatus.set_spp(SPP::User);
let mut cx = Self {
    x: [0; 32],
    sstatus,
    sepc: entry,
    kernel_satp,
    kernel_sp,
    trap_handler,
};
cx.set_sp(sp);
cx

#TODO: 整理设计模式
优化：没有未能实现的功能，可以不要 trait
*/
pub mod inner;

use spin::Mutex;
use crate::{ 
    file_system::{ Flag, file::File }, 
    concurrency::process::Process,
};
use inner::Scheduler;

/**
kernel process.
*/
pub trait Mod<S: Scheduler + 'static>: Dependence<S> {
    fn init() {
        let inner = S::new();

        let mut handler = Self::get_ref().lock();
        if let None = *handler {
            *handler = Some(inner);
        } else {
            panic!("Cannot reinitialize the scheduler.");
        }
    }

    fn access<F, V>(f: F) -> V 
    where
        F: FnOnce(&mut S) -> V,
    {
        let mut guard = Self::get_ref().lock();
        let option = guard.as_mut();
        if let Some(scheduler) = option {
            f(scheduler)
        } else { panic!("The scheduler is not initialized."); }
    }
}

pub trait Dependence<S: Scheduler> {
    fn open_file(name: &str, flag: Flag) -> Option<File>;
    fn get_ref() -> &'static Mutex<Option<S>>;
}
