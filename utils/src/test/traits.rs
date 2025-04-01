use crate::{print_serial, println_serial};

pub trait Testable {
    fn run(&self);
}

impl<T: Fn()> Testable for T {
    fn run(&self) {
        print_serial!("{}...\t", core::any::type_name::<T>());
        self();
        println_serial!("[ok]");
    }
}
