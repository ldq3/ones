/*!
介入机制需要的专用寄存器

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

pub mod data;

use crate::{ concurrency::thread::context::Context, memory::Address };
use data::Data;

pub trait Lib: Hal {
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

pub trait Hal {
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

    (handler_user, load_user_context, handler_kernel, load)

    fn layout() -> (usize, usize, usize, usize) {
        let relative_layout = Self::relative_layout(); 
        let base = Address::address(INTERVENE_TEXT);

        (base + relative_layout.0, base + relative_layout.1, base + relative_layout.2, base + relative_layout.3)
    }
    */
    fn layout() -> (usize, usize, usize, usize);
    /**
    set service routine.
    */
    fn service_set(address: usize);
    
    #[inline]
    fn dist_user(idata: &mut Data, cause: Cause, _value: usize) {
        use Cause::*;

        match cause {
            EnvCall => {
                Self::syscall(&idata.cx);
            },
            _ => { panic!("Unsupported trap!"); }
        }
    }
    /**
    service routine

    # 输入
    idata: intervene data
    */
    fn service_kernel(idata: &mut Data) {
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
                Self::breakpoiont(idata);
            }
            EnvCall => {
                Self::syscall(&idata.cx);
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
    /**
    intervene_data.cx.pc_add(4);
    let iid = intervene_data.cx.iid();
    let iarg = intervene_data.cx.iarg();

    // enable_supervisor_interrupt();

    let result = Self::syscall(iid, iarg);
    intervene_data.cx.iret_set(result as usize); // cx is changed during sys_exec, so we have to call it again
    */
    fn syscall(context: &Context) -> isize;
    /**
    intervene_data.cx.pc_add(2);
    */
    fn breakpoiont(idata: &mut Data);

    fn external() {
        todo!()
    }

    fn load_page(_number: usize) {
        todo!()
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