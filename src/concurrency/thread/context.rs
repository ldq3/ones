/**data registers

*/
pub trait Lib {
    /**
    intervene id
    */
    fn iid(context: &Context) -> usize;
    /**
    intervene return
    */
    fn iret(context: &Context) -> usize;
    /**
    intervene return
    */
    fn iret_set(context: &mut Context, value: usize);
    /**
    intervene argument
    */
    fn iarg(context: &Context) -> [usize; 3];

    fn sp_set(context: &mut Context, value: usize);
}

/**
何时设置 status
*/
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Context {
    pub data_reg: [usize; 32],
    pub status: usize,
    pub pc: usize,
}

impl Context {
    #[inline]
    pub fn new(status: usize, entry: usize) -> Self {
        Self { data_reg: [0; 32], status, pc: entry }
    }

    #[inline]
    pub fn empty() -> Self {
        Self {
            data_reg: [0; 32],
            status: 0,
            pc: 0
        }
    }
}