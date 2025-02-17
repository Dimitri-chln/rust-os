use x86_64::structures::idt::InterruptStackFrame;

use crate::interrupts::{index::InterruptIndex, pics::PICS};
use crate::print;

pub extern "x86-interrupt" fn handler(_stack_frame: InterruptStackFrame) {
    print!(".");

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}
