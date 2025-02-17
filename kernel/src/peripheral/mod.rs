pub mod console;
pub mod timer;

pub trait Peripheral {
    fn init();
}

pub struct Handler;

impl Peripheral for Handler {
    fn init() {
        use timer::Timer;
        timer::Handler::init();
    }
}

// mod config {
//     pub const MMIO: &[(usize, usize)] = &[
//         (0x0010_0000, 0x00_2000), // VIRT_TEST/RTC  in virt machine
//         (0x2000000, 0x10000),     // core local interrupter (CLINT)
//         (0xc000000, 0x210000),    // VIRT_PLIC in virt machine
//         (0x10000000, 0x9000),     // VIRT_UART0 with GPU  in virt machine
//     ];
// }