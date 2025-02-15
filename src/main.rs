#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use rust_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(not(test))]
    main();
    #[cfg(test)]
    test_main();

    loop {}
}

fn main() {
    println!("Hello World!");
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    rust_os::panic::handler(info)
}
