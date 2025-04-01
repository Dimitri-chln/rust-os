use core::panic::PanicInfo;

use drivers::println;
use utils::hlt::hlt_loop;

pub fn handler(info: &PanicInfo) -> ! {
    println!("{info}");
    hlt_loop();
}
