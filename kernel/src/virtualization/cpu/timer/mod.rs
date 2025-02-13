

pub trait TimerTrait {
    fn now() -> usize;

    fn init() {
        Self::set_next_trigger();
    }

    fn set_next_trigger();
}

pub static mut TICKS: usize = 0;

const TIMEBASW: usize = 100_000;
const FREQ: usize = 12_500_000;
const TICKS_PER_SEC: usize = 100;

use riscv::register::time;
use crate::virtualization::cpu::timer;

pub struct Timer;

impl timer::TimerTrait for Timer {
    fn now() -> usize {
        time::read()
    }
    
    fn set_next_trigger() {
        let next_trigger = Self::now() + FREQ / TICKS_PER_SEC;
        sbi_rt::set_timer(next_trigger as _);
    }
}
