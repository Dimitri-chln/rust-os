#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(utils::test::runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use drivers::println;
use utils::hlt::hlt_loop;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();
    hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    utils::test::panic::handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
