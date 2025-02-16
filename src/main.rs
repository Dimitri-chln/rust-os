#![no_std]
#![no_main]

use core::panic::PanicInfo;

use rust_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    rust_os::init();
    main();
    loop {}
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    rust_os::panic::handler(info)
}

fn main() {
    println!("Hello World!");
}
