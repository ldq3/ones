pub mod instance;
pub mod virtio;
pub mod plic;

use core::any::Any;

pub trait Lib {
    /**
    Handle exception.
    */
    fn handle();

    fn init();
}

/**
for Arc, can not be mut
*/
pub trait Block: Send + Sync + Any {
    fn read(&mut self, address: usize, cache: &mut [u8]);
    fn write(&mut self, address: usize, cache: &[u8]);
}

pub trait Character {
    fn read(&mut self) -> u8;
    fn write(&mut self, char: u8);
}

pub trait Network {

}

pub trait Bus {
    
}