pub mod exception;
pub mod timer;

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