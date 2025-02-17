/*!
pub struct Contexts {
    address_space: usize,

    // program status
    program_counter: usize,
    stack_pointer: usize,
    data_registers: [usize; 32],

    // machine status
    machine_status: usize,
}
*/

use riscv::register::sstatus::{self, Sstatus};

#[repr(C)]
pub struct Context {
    _x: [usize; 32], // 通用寄存器组
    _sstatus: Sstatus, // 状态寄存器
    pub sepc: usize, // Exception Program Counter
    _kernel_satp: usize,
    _kernel_sp: usize,
    _trap_handler: usize,
}

impl Context {
    // FIXME
    pub fn new() -> Self {
        Self {
            _x: [0; 32],
            _sstatus: sstatus::read(),
            sepc: 0,
            _kernel_satp: 0,
            _kernel_sp: 0,
            _trap_handler: 0,
        }
    }

    // pub fn set_sp(&mut self, sp: usize) {
    //     self.x[2] = sp;
    // }

    // pub fn inc_epc(&mut self, n: usize) {
    //     self.sepc += n;
    // }

    // // function support
    // pub fn set_ret(&mut self, ret: usize) {
    //     self.x[10] = ret;
    // }

    // pub fn fn_args(&self) -> [usize; 3] {
    //     [ self.x[10], self.x[11], self.x[12] ]
    // }

    // pub fn syscall_id(&self) -> usize {
    //     self.x[17]
    // }
}
