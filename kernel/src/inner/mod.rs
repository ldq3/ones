pub mod cpu;
mod memory;

pub fn init() {
    cpu::init();
    memory::init_heap();
    memory::heap_test();
}