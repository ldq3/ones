/**
ra: 函数的返回地址（return address）
*/
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Context {
    pub ra: usize,
    pub sp: usize,
    /// saved register
    pub s: [usize; 12],
}

impl Context {
    #[inline]
    pub fn new(ra: usize, sp: usize) -> Self {
        Self {
            ra,
            sp,
            s: [0; 12]
        }
    }
    /**
    empty context
    */
    #[inline]
    pub fn empty() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12]
        }
    }
}
