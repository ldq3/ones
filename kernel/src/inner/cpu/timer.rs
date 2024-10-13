pub use crate::inner::arch_ins::cpu::timer;

pub trait Timer {
    fn now() -> usize;

    fn set_next_trigger();
}

pub fn init() {
    timer::Timer::set_next_trigger();
}
