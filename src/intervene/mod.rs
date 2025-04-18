/*!
# 上下文切换
内核的 intervene 执行流

切换的内容：
- 数据（sp)：scratch
- 地址空间
- 代码（pc）

- 内核栈
- 用户栈
- intervene data
- intervene text
*/

pub mod context;

use crate::{cpu::DataReg, memory::Address};
use crate::runtime::address_space::config::INTERVENE_TEXT;

pub trait Lib<DG: DataReg + 'static>: Dependence<DG> {
    /**
    set kernel trap entry
    alltraps_k
    trap_from_kernel
    trap_return: set user trap entry

    run task
    init_process
    new process: 直接将 process 加入调度器，go to trap_return
    */
    fn init();
    /**
    service routine
    */
    fn service_user();

    fn return_to_user() -> !;
}

pub trait Dependence<DG: DataReg> {
    /**Get Exception cause.
    
    PlatformDependent
    */
    fn cause() -> Cause;
    /**Get Exception value.
    
    PlatformDependent
    */
    fn value() -> usize;
    /**
    Set handler.
    */
    fn handler_set(address: usize);
    /**
    Return the relative memory layout of asm function.
    */
    fn relative_layout() -> (usize, usize, usize, usize);
    /**
    (handler_user, load_user_context, handler_kernel, load)
    */
    fn layout() -> (usize, usize, usize, usize) {
        let relative_layout = Self::relative_layout(); 
        let base = Address::address(INTERVENE_TEXT);

        (base + relative_layout.0, base + relative_layout.1, base + relative_layout.2, base + relative_layout.3)
    }
    /**
    set service routine.
    */
    fn service_set(address: usize);
    
    #[inline]
    fn dist_user(intervene_data: &mut Data<DG>, cause: Cause, _value: usize) {
        use Cause::*;

        match cause {
            EnvCall => {
                intervene_data.pc +=4;
                let data_reg = &mut intervene_data.data_reg;

                // enable_supervisor_interrupt();

                let result = Self::syscall(data_reg.iid(), data_reg.iarg());
                data_reg.iret_set(result as usize); // cx is changed during sys_exec, so we have to call it again
            },
            _ => { panic!("Unsupported trap!"); }
        }
    }
    /**
    service routine
    */
    fn service_kernel(intervene_data: &mut Data<DG>) {
        use Cause::*;
        let cause = Self::cause();
        let _value = Self::value();

        match cause {
            // SupervisorTimer => {
            //     use crate::peripheral::timer::{ self, Timer };
            //     timer::Handler::set_next_trigger();
            //     timer::Handler::check();
            //     info!("Timer.");
            // },
            Breakpoint => {
                intervene_data.pc += 2;
            }
            EnvCall => {
                intervene_data.pc +=4;

                // enable_supervisor_interrupt();
                let data_reg = &mut intervene_data.data_reg;

                let result = Self::syscall(data_reg.iid(), data_reg.iarg());
                
                // cx is changed during sys_exec, so we have to call it again
                data_reg.iret_set(result as usize);
            }
            External => {
                Self::external();
            },
            PageLoadFault => {
                let address = Self::value();
                let page_number = Address::number(address);
                Self::load_page(page_number);
            },
            Unknown => {
                let value = Self::value();
                panic!("Unsupported trap, value: 0x{:x}", value);
            }
        }
    }

    fn syscall(id: usize, args: [usize; 3]) -> isize;

    fn external() {
        todo!()
    }

    fn load_page(_number: usize) {
        todo!()
    }
}

use context::KernelInfo;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Data<DG: DataReg> {
    pub data_reg: DG,
    pub status: usize,
    pub pc: usize,

    pub kernel_info: KernelInfo,
}

impl<DG: DataReg> Data<DG> {
    #[inline]
    pub fn get_mut(frame_number: usize) -> &'static mut Self {
        use crate::memory::Address;
        let address = Address::address(frame_number);
        unsafe{ &mut *(address as *mut Self) }
    }
}

#[derive(Debug)]
pub enum Cause {
    EnvCall,
    Breakpoint,
    External,
    PageLoadFault,

    Unknown
}
