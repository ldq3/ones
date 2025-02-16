use cpu::Handler;

pub mod cpu;
// pub mod memory;
pub mod process;
pub mod syscall;
mod memory;

pub fn init() {
    use memory::Memory;
    memory::Handler::init();

    // use cpu::CentralProcessUnit;
    // Handler::init();
}

#[cfg(test)]
pub mod test {
    pub fn main() {
        use super::cpu;
        cpu::test::main();
    }
}
