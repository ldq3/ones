use core::fmt::{ self, Write };

pub fn console_putchar(c: usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

// add escape sequence of color to string
// #[macro_export]
// macro_rules! color {
//     ($args: ident, $color_code: ident) => {
//         format_args!("\u{1B}[{}m{}\u{1B}[0m", $color_code as u8, $args)
//     };
// }