use core::fmt;
use core::fmt::Write;

use x86_64::instructions::interrupts;

use crate::vga_buffer::WRITER;

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[macro_export]
macro_rules! print_vga {
    ($($arg:tt)*) => ($crate::vga_buffer::macros::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println_vga {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_vga!("{}\n", format_args!($($arg)*)));
}
