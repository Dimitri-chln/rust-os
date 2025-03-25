use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::gdt;
use crate::interrupts::handlers::*;
use crate::interrupts::index::InterruptIndex;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // Exceptions
        idt.breakpoint.set_handler_fn(breakpoint::handler);
        idt.page_fault.set_handler_fn(page_fault::handler);

        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault::handler)
                .set_stack_index(gdt::tss::DOUBLE_FAULT_IST_INDEX);
        }

        // Interrupts
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt::handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt::handler);

        idt
    };
}

pub fn init() {
    IDT.load();
}
