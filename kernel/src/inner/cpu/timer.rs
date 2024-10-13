pub trait Timer {
    fn now() -> usize;

    fn set_next_trigger();
}

pub fn init() {
    TimerIns::set_next_trigger();
}

pub use crate::inner::arch_ins::cpu::timer::TimerRv as TimerIns;