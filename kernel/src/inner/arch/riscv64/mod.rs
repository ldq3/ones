use core::arch::global_asm;

pub mod cpu;
mod memory;
mod process;

global_asm!(include_str!("entry.asm"));

pub fn shutdown(failure: bool) -> ! {
    #[allow(deprecated)]
    use sbi_rt::{ system_reset, NoReason, Shutdown, SystemFailure };

    if !failure {
        system_reset(Shutdown, NoReason);
    } else {
        system_reset(Shutdown, SystemFailure);
    }

    unreachable!()
}