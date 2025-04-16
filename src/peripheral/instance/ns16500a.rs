/*!
| I/O Port | Read (DLAB=0)            | Write (DLAB=0)           | Read (DLAB=1)            | Write (DLAB=1)           |
|----------|--------------------------|--------------------------|--------------------------|--------------------------|
| base     | RBR (Receiver Buffer)    | THR (Transmitter Holding)| DLL (Divisor Latch LSB)  | DLL (Divisor Latch LSB)  |
| base+1   | IER (Interrupt Enable)   | IER (Interrupt Enable)   | DLM (Divisor Latch MSB)  | DLM (Divisor Latch MSB)  |
| base+2   | IIR (Interrupt Identify) | FCR (FIFO Control)       | IIR (Interrupt Identify) | FCR (FIFO Control)       |
| base+3   | LCR (Line Control)       | LCR (Line Control)       | LCR (Line Control)       | LCR (Line Control)       |
| base+4   | MCR (Modem Control)      | MCR (Modem Control)      | MCR (Modem Control)      | MCR (Modem Control)      |
| base+5   | LSR (Line Status)        | Factory Test             | LSR (Line Status)        | Factory Test             |
| base+6   | MSR (Modem Status)       | Not Used                 | MSR (Modem Status)       | Not Used                 |
| base+7   | SCR (Scratch)            | SCR (Scratch)            | SCR (Scratch)            | SCR (Scratch)            |
*/

use crate::peripheral::Character;

extern crate alloc;
use alloc::collections::VecDeque;

struct Register(&'static mut [u8]);

impl Register {
    #[inline]
    fn rbr(&self) -> u8 {
        self.0[0]
    }

    #[inline]
    fn lsr(&self) -> LSR {
        LSR{ bits: self.0[5] }
    }

    // write
    #[inline]
    fn thr(&mut self) -> &mut u8 {
        &mut self.0[0]
    }
}

use bitflags::bitflags;
bitflags! {
    /// InterruptEnableRegister
    pub struct IER: u8 {
        const RX_AVAILABLE = 1 << 0;
        const TX_EMPTY = 1 << 1;
    }

    /// LineStatusRegister
    pub struct LSR: u8 {
        const DATA_AVAILABLE = 1 << 0;
        const THR_EMPTY = 1 << 5;
    }

    /// Model Control Register
    pub struct MCR: u8 {
        const DATA_TERMINAL_READY = 1 << 0;
        const REQUEST_TO_SEND = 1 << 1;
        const AUX_OUTPUT1 = 1 << 2;
        const AUX_OUTPUT2 = 1 << 3;
    }
}

pub struct Handler {
    register: Register,
    read_buffer: VecDeque<u8>
}

impl Character for Handler {
    fn read(&mut self) -> u8 {
        loop {
            if let Some(ch) = self.read_buffer.pop_front() { return ch; }
            // } else {
            //     let task_cx_ptr = self.condvar.wait_no_sched();
            //     drop(inner);
            //     schedule(task_cx_ptr);
            // }
        }
    }

    fn write(&mut self, char: u8) {
        loop {
            if self.register.lsr().contains(LSR::THR_EMPTY) {
                *(self.register.thr()) = char;
                break;
            }
        }
    }
}

impl Handler {
    // pub fn new(memory_mapped_area: MemoryMappedArea<u8, 8>) -> Self {
    //     Self {
    //         register: Register(memory_mapped_area),
    //         read_buffer: VecDeque::new(),
    //     }
    // }

    pub fn read_handler(&self) -> Option<u8> {
        if self.register.lsr().contains(LSR::DATA_AVAILABLE) {
            Some(self.register.rbr())
        } else {
            None
        }
    }
}
