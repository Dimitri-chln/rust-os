use core::panic::PanicInfo;

use crate::test::qemu;
use crate::{hlt, println_serial};

pub fn handler(info: &PanicInfo) -> ! {
    println_serial!("[failed]\n");
    println_serial!("Error: {info}\n");
    qemu::exit(qemu::ExitCode::Failed);
    hlt::hlt_loop();
}
