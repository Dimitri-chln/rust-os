use crate::println_serial;
use crate::test::qemu;

use super::traits::Testable;

pub fn runner(tests: &[&dyn Testable]) {
    println_serial!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    qemu::exit(qemu::ExitCode::Success);
}
