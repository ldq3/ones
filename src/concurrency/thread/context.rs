/**data registers

*/
pub trait Context {
    fn empty() -> Self;
    /**
    intervene id
    */
    fn iid(&self) -> usize;
    /**
    intervene return
    */
    fn iret(&self) -> usize;
    /**
    intervene return
    */
    fn iret_set(&mut self, value: usize);
    /**
    intervene argument
    */
    fn iarg(&self) -> [usize; 3];

    fn sp_set(&mut self, value: usize);

    fn pc_add(&mut self, value: usize);
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ModelContext {
    pub data_reg: [usize; 32],
    pub status: usize,
    pub pc: usize,
}

impl ModelContext {
    #[inline]
    pub fn empty() -> Self {
        Self {
            data_reg: [0; 32],
            status: 0,
            pc: 0
        }
    }
}