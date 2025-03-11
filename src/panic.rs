use core::panic::PanicInfo;

use crate::{println_vga, utils};

pub fn handler(info: &PanicInfo) -> ! {
    println_vga!("{info}");
    utils::hlt_loop();
}
