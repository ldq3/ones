pub trait HandlerTrait {
    fn now() -> usize;

    fn set_next_trigger();
}

pub fn init() {
    Handler::set_next_trigger();
}

pub use crate::arch_ins::driver::timer::*;