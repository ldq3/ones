mod arch;

pub trait TimerTrait {
    fn now() -> usize;

    fn init() {
        Self::set_next_trigger();
    }

    fn set_next_trigger();
}

pub use arch::riscv64::Timer;
