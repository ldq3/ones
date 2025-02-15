#![allow(unused)]

pub mod timer;
pub mod exception;
pub mod context;

pub trait CentralProcessUnit {
    fn init() {
        use exception::*;
        Handler::init();

        use timer::*;
        Timer::init();  
    }

    fn shutdown(failure: bool) -> !;
}

pub mod test {
    pub fn main() {
        use super::exception;
        exception::test::main();
    }
}

pub struct Handler;

impl CentralProcessUnit for Handler {
    fn shutdown(failure: bool) -> ! {
        #[allow(deprecated)]
        use sbi_rt::{ system_reset, NoReason, Shutdown, SystemFailure };

        if !failure {
            system_reset(Shutdown, NoReason);
        } else {
            system_reset(Shutdown, SystemFailure);
        }

        unreachable!() 
    }
}