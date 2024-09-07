pub use crate::arch_ins::driver::console::*;

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::arch_ins::driver::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::arch_ins::driver::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    };
}