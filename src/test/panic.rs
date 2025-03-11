use core::panic::PanicInfo;

use crate::test::qemu;
use crate::{serial_println, utils};

pub fn handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {info}\n");
    qemu::exit(qemu::ExitCode::Failed);
    utils::hlt_loop();
}
