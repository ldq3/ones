use core::arch::global_asm;
use crate::exception::{
    Context as TrapContext,
    Handler as TrapHandler,
};
use log::info;
use riscv::register::{
    scause, sepc, sstatus::{ self, Sstatus, SPP }, stvec::{ self, TrapMode }
};
use crate::println;

pub struct Context {
    x: [usize; 32],
    sstatus: Sstatus,
    sepc: usize,
}

global_asm!(include_str!("context.S"));

impl TrapContext for Context { 
    fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    fn inc_epc(&mut self, n: usize) {
        self.sepc += n;
    }

    fn set_ret(&mut self, ret: usize) {
        self.x[10] = ret;
    }

    fn fn_args(&self) -> [usize; 3] {
        [ self.x[10], self.x[11], self.x[12] ]
    }

    fn syscall_id(&self) -> usize {
        self.x[17]
    }
}

pub struct Handler;

impl TrapHandler<Context> for Handler {
    fn init() {
        unsafe {
            stvec::write(Self::call_sys as usize, TrapMode::Direct);
        }

        info!("init trap handler")
    }
    
    fn into_user() {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);

        let mut cx = Context {
            x: [0; 32],
            sstatus,
            sepc: 0, // FIXME: the sepc should be the first instruction of user app
        };

        cx.set_sp(0);

        Self::ret_user(0);
    }   

    fn call_sys() {
        extern "C" { fn __call_sys(); }
        unsafe{
            __call_sys();
        }
    } 

    #[no_mangle]
    fn distribute(_cx: &mut Context) -> ! {
        let scause = scause::read().cause();
        // let stval = stval::read(),
        let sepc = sepc::read();
        println!("trap: cause: {:?}, epc: 0x{:#x}", scause, sepc);
        panic!("trap handled!");

        // match scause.cause() {
            // Trap::Exception(Exception::UserEnvCall) => {
                // cx.inc_epc(4);
                // cx.set_ret(
                    // trap::syscall::syscall(cx.syscall_id(), cx.fn_args()) as usize
                // );
            // },
            // _ => {}
        // }
    }
    
    fn ret_user(_cx_addr: usize) {
        extern "C" { fn __ret_user(cx_addr: usize); }
        unsafe {
            __ret_user(0);
        }        
    }
}
