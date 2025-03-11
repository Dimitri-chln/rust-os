#![no_std]
#![feature(abi_x86_interrupt)]
// Tests
#![cfg_attr(test, no_main)]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(crate::test::runner))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]

extern crate alloc;

pub mod allocator;
pub mod gdt;
mod init;
pub mod interrupts;
pub mod macros;
pub mod memory;
pub mod panic;
pub mod serial;
pub mod task;
pub mod utils;
pub mod vga_buffer;

pub use init::init;

pub mod test;
cfg_test! {
    use core::panic::PanicInfo;

    use bootloader::{entry_point, BootInfo};

    entry_point!(kernel_main);

    fn kernel_main(_boot_info: &'static BootInfo) -> ! {
        init();
        test_main();
        utils::hlt_loop();
    }

    #[panic_handler]
    fn panic_handler(info: &PanicInfo) -> ! {
        test::panic::handler(info)
    }
}
