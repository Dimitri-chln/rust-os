use core::panic::PanicInfo;

use crate::println;

pub fn handler(info: &PanicInfo) -> ! {
    println!("{info}");
    crate::hlt_loop();
}
