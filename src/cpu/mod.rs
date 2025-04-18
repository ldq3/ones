pub trait Lib {
    fn shutdown(failure: bool) -> !;
    /**
    启用分页内存管理
    */
    fn page_enable(bits: usize);
    /**启用 Platform-Level Interrupt Controller

    PlatformDependent
    */
    fn plic_enable();
}

/**data registers

*/
pub trait DataReg {
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
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ModelDataReg(pub [usize; 32]);