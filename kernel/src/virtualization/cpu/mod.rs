pub mod timer;
pub mod exception;
pub mod context;

pub trait CentralProcessUnit {
    fn init() {
        use exception::Exception;
        exception::Handler::init();

        use timer::Timer;
        timer::Handler::init();  
    }

    fn shutdown(failure: bool) -> !;
}

pub struct Handler;

impl CentralProcessUnit for Handler {
    fn shutdown(failure: bool) -> ! {
        #[allow(deprecated)]
        use sbi_rt::{ system_reset, NoReason, Shutdown, SystemFailure };

        if !failure {
            system_reset(Shutdown, NoReason);
        } else {
            system_reset(Shutdown, SystemFailure);
        }

        unreachable!() 
    }
}

#[cfg(test)]
mod test {
    pub fn main() {
        
    }
}