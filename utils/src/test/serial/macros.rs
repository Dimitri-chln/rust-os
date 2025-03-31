use core::fmt::Write;

use x86_64::instructions::interrupts;

use super::SERIAL1;

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    interrupts::without_interrupts(|| {
        SERIAL1
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
    });
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! print_serial {
    ($($arg:tt)*) => {
        $crate::test::serial::macros::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! println_serial {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::print_serial!("{}\n", format_args!($($arg)*)));
}
