pub static mut TICKS: usize = 0;

const TIMEBASW: usize = 100_000;
const FREQ: usize = 12_500_000;
const TICKS_PER_SEC: usize = 100;

use riscv::register::time;
use crate::inner::cpu::timer;

pub struct Handler{}

impl timer::HandlerTrait for Handler {
    fn now() -> usize {
        time::read()
    }
    
    fn set_next_trigger() {
        let next_trigger = Self::now() + FREQ / TICKS_PER_SEC;
        sbi_rt::set_timer(next_trigger as _);
    }
}
 