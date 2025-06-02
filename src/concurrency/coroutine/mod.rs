pub mod context;

use alloc::vec;
use alloc::vec::Vec;
use context::Context;

use super::scheduler::Scheduler as S;

#[derive(Clone)]
pub struct Coroutine {
    pub id: usize,
    pub cx: Context
}

impl Coroutine {
    pub fn new(cx: Context) -> usize {
        access(|scheduler| {
            let id = scheduler.id.add();

            let coroutine = Self {
                id,
                cx
            };
            scheduler.coroutine[id] = Some(coroutine);

            id
        })
    }

    #[inline]
    fn empty() -> Self {
        Self { id: 0, cx: Context::empty() }
    }
}

pub trait Lib: Dep {
    /**
    在 idle 内核控制流中使用，保存当前内核控制流上下文，并切换至由 (pid, tid) 指定的用户程序内核 intervene 控制流。
    */
    fn switch_to_ready() {
        access(|scheduler| {
            let coroutine = scheduler.coroutine[0].as_mut().unwrap();
            let idle = &mut coroutine.cx as *mut Context;

            let cid = scheduler.id.switch_s();
            let coroutine = scheduler.coroutine[cid].as_ref().unwrap();
            let next = &coroutine.cx as *const Context;

            Self::switch(idle, next);
        })
    }
    /**
    由当前用户程序内核 intervene 控制流切换至 idle 控制流
    */
    fn switch_to_idle() {
        access(|scheduler| {
            let cid = scheduler.id.running.unwrap();
            let coroutine = scheduler.coroutine[cid].as_mut().unwrap();
            let current = &mut coroutine.cx as *mut Context;

            let coroutine = scheduler.coroutine[0].as_ref().unwrap();
            let idle = &coroutine.cx as *const Context;

            Self::switch(current, idle);
        })
    }
}

pub trait Dep {
    /**
    Save current context and load next context.
    */
    fn switch(current: *mut Context, next: *const Context);
}

pub struct Scheduler {
    pub coroutine: Vec<Option<Coroutine>>,
    pub id: S
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref SCHEDULER: Mutex<Scheduler> = {
        let mut scheduler = Scheduler {
            coroutine: vec![None; 16],
            id: S::new(15)
        };

        scheduler.coroutine[0] = Some(Coroutine::empty());

        Mutex::new(scheduler)
    };
}

/**
Access coroutine scheduler.
*/
#[inline]
pub fn access<F, V>(f: F) -> V
where
    F: FnOnce(&mut Scheduler) -> V,
{
    let mut mutex = SCHEDULER.lock();
    f(&mut mutex)
}
