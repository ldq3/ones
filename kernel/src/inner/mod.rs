pub mod cpu;
pub mod memory;
pub mod process;
pub mod syscall;

pub fn init() {
    use cpu::*; // #FIXME: 如何自动捆绑导入？

    CentralProcessUnit::init(); 
}
