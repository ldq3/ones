pub mod timer;
pub mod exception;
pub mod context;

mod arch;

pub trait CentralProcessUnitTrait {
    fn init() {
        use exception::*;
        Handler::init();

        use timer::*;
        Timer::init();  
    }

    fn shutdown(failure: bool) -> !;
}

pub use arch::riscv64::CentralProcessUnit;
