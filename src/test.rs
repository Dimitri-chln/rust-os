pub mod panic;
mod qemu;
pub mod traits;

use traits::Testable;

use crate::serial_println;

pub fn runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    qemu::exit(qemu::ExitCode::Success);
}
