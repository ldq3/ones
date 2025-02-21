/**
为方便汇编程序编写，增加以下字段：
- kernel_satp
- kernel_sp
- trap_handler
*/
#[repr(C)]
pub struct Context {
    pub x: [usize; 32], // 通用寄存器组
    sstatus: usize, // 状态寄存器
    pub sepc: usize, // Exception Program Counter

    kernel_satp: usize,
    kernel_sp: usize,
    trap_handler: usize,
}

use ones::exception::context::Context as C;

impl C for Context {
    #[inline]
    fn pc(&mut self) -> &mut usize {
        &mut self.sepc
    }

    #[inline]
    fn eargs(&self) -> [usize; 3] {
        self.x[10..=12].try_into().expect("Wrong slice length.")
    }

    #[inline]
    fn eid(&mut self) -> &mut usize {
        &mut self.x[17]
    }

    #[inline]
    fn eret(&mut self) -> &mut usize {
        &mut self.x[10]
    }
}