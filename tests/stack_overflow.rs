#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test::runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use rust_os::serial_print;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");

    rust_os::gdt::init();
    test_idt::init();

    // trigger a stack overflow
    stack_overflow();

    panic!("Execution continued after stack overflow");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test::panic::handler(info)
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed
    volatile::Volatile::new(0).read(); // prevent tail recursion optimizations
}

mod test_idt {
    use lazy_static::lazy_static;
    use rust_os::{serial_println, test::qemu};
    use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

    lazy_static! {
        static ref IDT: InterruptDescriptorTable = {
            let mut idt = InterruptDescriptorTable::new();

            unsafe {
                idt.double_fault
                    .set_handler_fn(double_fault_handler)
                    .set_stack_index(rust_os::gdt::tss::DOUBLE_FAULT_IST_INDEX);
            }

            idt
        };
    }

    pub fn init() {
        IDT.load();
    }

    extern "x86-interrupt" fn double_fault_handler(
        _stack_frame: InterruptStackFrame,
        _error_code: u64,
    ) -> ! {
        serial_println!("[ok]");

        qemu::exit(qemu::ExitCode::Success);
        rust_os::hlt_loop();
    }
}
