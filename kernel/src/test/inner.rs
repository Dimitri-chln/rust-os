use crate::serial_println;
use crate::test::qemu;

use super::traits::Testable;

pub fn runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    qemu::exit(qemu::ExitCode::Success);
}
