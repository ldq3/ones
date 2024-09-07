use crate::{ print, println };

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        WRITE => write(args[0], args[1] as *const u8, args[2]),
        EXIT => exit(args[0] as i32),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}

const WRITE: usize = 64;
const EXIT: usize = 93;
const FD_STDOUT: usize = 1;

pub fn write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let slice = unsafe {
                core::slice::from_raw_parts(buf, len)
            };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        },
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}

pub fn exit(xstate: i32) -> ! {
    println!("[kernel] Application exited with code {}", xstate);
    panic!("process exit");
}