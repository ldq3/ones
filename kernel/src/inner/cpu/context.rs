pub struct Contexts {
    address_space: usize,

    // program status
    program_counter: usize,
    stack_pointer: usize,
    data_registers: [usize; 32],

    // machine status
    machine_status: usize,
}

pub trait Context {
    fn set_sp(&mut self, sp: usize);

    fn inc_epc(&mut self, n: usize);

    fn set_ret(&mut self, ret: usize);

    fn fn_args(&self) -> [usize; 3];

    fn syscall_id(&self) -> usize;
}