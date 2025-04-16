pub trait Lib: Dependence {
    /**
    只会在一处使用，所以标记为 inline。
    */
    #[inline]
    fn syscall(id: usize, _args: [usize; 3]) -> isize {
        match id {
            _ => panic!("Unsupported syscall id: {}.", id),
        }
    }

    // fn write(fd: usize, buf: *const u8, len: usize) -> isize {
    //     let token = Self::current_user_token();
    //     let task = current_task().unwrap();
    //     let inner = task.inner_exclusive_access();
    //     if fd >= inner.fd_table.len() {
    //         return -1;
    //     }
    //     if let Some(file) = &inner.fd_table[fd] {
    //         let file = file.clone();
    //         // release current task TCB manually to avoid multi-borrow
    //         drop(inner);
    //         file.write(
    //             UserBuffer::new(translated_byte_buffer(token, buf, len))
    //         ) as isize
    //     } else {
    //         -1
    //     }
    // }
}

pub trait Dependence {
    fn current_user_token() -> usize;
}

pub mod config {
    
    /// Duplicate File Descriptor
    pub const DUP: usize = 24;

    pub const CONNECT: usize = 29;
    pub const LISTEN: usize = 30;
    pub const ACCEPT: usize = 31;
    pub const OPEN: usize = 56;
    pub const CLOSE: usize = 57;
    pub const PIPE: usize = 59;
    pub const READ: usize = 63;
    pub const WRITE: usize = 64;
    pub const EXIT: usize = 93;
    pub const SLEEP: usize = 101;
    pub const YIELD: usize = 124;
    pub const KILL: usize = 129;
    pub const GET_TIME: usize = 169;
    pub const GETPID: usize = 172;
    pub const FORK: usize = 220;
    pub const EXEC: usize = 221;
    pub const WAITPID: usize = 260;
    pub const THREAD_CREATE: usize = 1000;
    pub const GETTID: usize = 1001;
    pub const WAITTID: usize = 1002;
    pub const MUTEX_CREATE: usize = 1010;
    pub const MUTEX_LOCK: usize = 1011;
    pub const MUTEX_UNLOCK: usize = 1012;
    pub const SEMAPHORE_CREATE: usize = 1020;
    pub const SEMAPHORE_UP: usize = 1021;
    pub const SEMAPHORE_DOWN: usize = 1022;
    pub const CONDVAR_CREATE: usize = 1030;
    pub const CONDVAR_SIGNAL: usize = 1031;
    pub const CONDVAR_WAIT: usize = 1032;
    pub const FRAMEBUFFER: usize = 2000;
    pub const FRAMEBUFFER_FLUSH: usize = 2001;
    pub const EVENT_GET: usize = 3000;
    pub const KEY_PRESSED: usize = 3001;
}