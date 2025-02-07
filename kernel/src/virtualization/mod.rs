pub mod cpu;
// pub mod memory;
pub mod process;
pub mod syscall;
mod memory;

pub fn init() {
    use cpu::*; // #FIXME: 如何自动捆绑导入？

    CentralProcessUnit::init(); 
}

pub mod test {
    pub fn main() {
        use super::cpu;
        cpu::test::main();
    }
}