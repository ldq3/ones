use crate::inner::cpu::context;

use riscv::register::sstatus::Sstatus;

pub struct Context {
    x: [usize; 32],
    sstatus: Sstatus,
    pub sepc: usize,
}

impl context::ContextTrait for Context { 
    fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    fn inc_epc(&mut self, n: usize) {
        self.sepc += n;
    }

    // function support
    fn set_ret(&mut self, ret: usize) {
        self.x[10] = ret;
    }

    fn fn_args(&self) -> [usize; 3] {
        [ self.x[10], self.x[11], self.x[12] ]
    }

    fn syscall_id(&self) -> usize {
        self.x[17]
    }
}