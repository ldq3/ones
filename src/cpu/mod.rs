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