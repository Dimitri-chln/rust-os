#![no_std]
#![cfg_attr(test, no_main)]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(crate::test::runner))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]

pub mod macros;
pub mod panic;
pub mod serial;
pub mod vga_buffer;

pub mod test;

cfg_test! {
    use core::panic::PanicInfo;
}

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    test::panic::handler(info)
}
