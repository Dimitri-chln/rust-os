use core::fmt;
use core::fmt::Write;

use x86_64::instructions::interrupts;

use super::WRITER;

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        if let Some(writer) = WRITER.r#try() {
            writer.lock().write_fmt(args).unwrap();
        }
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::display::frame_buffer::macros::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
