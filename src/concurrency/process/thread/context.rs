#[derive(Clone, Debug)]
#[repr(C)]
pub struct Context {
    /// return address
    pub pc: usize,
    /// kernel stack pointer
    pub sp: usize,
    /// saved register
    pub s: [usize; 12],
}

impl Context {
    #[inline]
    pub fn new(sp: usize, ra: usize) -> Self {
        Self { pc: ra, sp, s: [0; 12] }
    }

    #[inline]
    pub fn empty() -> Self {
        Self {
            pc: 0,
            sp: 0,
            s: [0; 12]
        }
    }
}