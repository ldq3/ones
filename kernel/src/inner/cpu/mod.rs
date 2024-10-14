pub mod timer;
pub mod exception;
pub mod context;

pub fn init() {
    exception::init();
    timer::init();  
}

// fn into_user() {
    // let mut sstatus = sstatus::read();
    // sstatus.set_spp(SPP::User);

    // let mut cx = Context {
        // x: [0; 32],
        // sstatus,
        // sepc: 0, // FIXME: the sepc should be the first instruction of user app
    // };

    // cx.set_sp(0);

    // Self::expt_ret(0);
// }