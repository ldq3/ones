pub struct Contexts {
    address_space: usize,

    // program status
    program_counter: usize,
    stack_pointer: usize,
    data_registers: [usize; 32],

    // machine status
    machine_status: usize,
}