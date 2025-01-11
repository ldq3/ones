mod arch;

pub trait ContextTrait {
    fn set_sp(&mut self, sp: usize);

    fn inc_epc(&mut self, n: usize);

    fn set_ret(&mut self, ret: usize);

    fn fn_args(&self) -> [usize; 3];

    fn syscall_id(&self) -> usize;
}

pub use arch::riscv64::Context;