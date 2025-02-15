use core::panic::PanicInfo;

use crate::{serial_println, test::qemu};

pub fn handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {info}\n");
    qemu::exit(qemu::ExitCode::Failed);
    loop {}
}
