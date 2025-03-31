#![no_std]
#![feature(abi_x86_interrupt)]
// Tests
#![cfg_attr(test, no_main)]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(utils::test::runner))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]

extern crate alloc;

pub mod allocator;
pub mod gdt;
mod init;
pub mod interrupts;
pub mod macros;
pub mod memory;
pub mod panic;

pub use init::init;

cfg_test! {
    use core::panic::PanicInfo;

    use bootloader_api::{entry_point, BootInfo};

    entry_point!(kernel_main);

    fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
        init(&mut boot_info.framebuffer);
        test_main();
        utils::hlt::hlt_loop();
    }

    #[panic_handler]
    fn panic_handler(info: &PanicInfo) -> ! {
        utils::test::panic::handler(info)
    }
}
