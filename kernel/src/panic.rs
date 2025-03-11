use core::panic::PanicInfo;

use crate::println_vga;

pub fn handler(info: &PanicInfo) -> ! {
    println_vga!("{info}");
    crate::hlt_loop();
}
