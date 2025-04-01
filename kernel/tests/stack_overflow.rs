#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test::runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use utils::print_serial;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_serial!("stack_overflow::stack_overflow...\t");

    kernel::gdt::init();
    test_idt::init();

    // trigger a stack overflow
    stack_overflow();

    panic!("Execution continued after stack overflow");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    utils::test::panic::handler(info)
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed
    volatile::Volatile::new(0).read(); // prevent tail recursion optimizations
}

mod test_idt {
    use lazy_static::lazy_static;
    use utils::hlt::hlt_loop;
    use utils::{println_serial, test::qemu};
    use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

    lazy_static! {
        static ref IDT: InterruptDescriptorTable = {
            let mut idt = InterruptDescriptorTable::new();

            unsafe {
                idt.double_fault
                    .set_handler_fn(double_fault_handler)
                    .set_stack_index(kernel::gdt::tss::DOUBLE_FAULT_IST_INDEX);
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
        println_serial!("[ok]");

        qemu::exit(qemu::ExitCode::Success);
        hlt_loop();
    }
}
