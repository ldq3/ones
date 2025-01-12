pub fn satp_from_page_table() -> usize {
    8usize << 60 // | self.root_ppn.0
}

use crate::inner::cpu;

pub struct CentralProcessUnit {}

impl cpu::CentralProcessUnitTrait for CentralProcessUnit {
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
