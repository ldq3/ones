pub mod timer;
pub mod exception;

pub fn init() {
    exception::init();
    timer::init();  
}