pub static mut TICKS: usize = 0;

const TIMEBASW: usize = 100_000;
const FREQ: usize = 12_500_000;
const TICKS_PER_SEC: usize = 100;

use riscv::register::time;

pub fn now() -> usize {
    time::read()
}

pub fn set_next_trigger() {
    let next_trigger = now() + FREQ / TICKS_PER_SEC;
    sbi_rt::set_timer(next_trigger as _);
}