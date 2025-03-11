use x86_64::instructions::port::Port;
use x86_64::structures::idt::InterruptStackFrame;

use crate::interrupts::index::InterruptIndex;
use crate::interrupts::pics::PICS;

pub extern "x86-interrupt" fn handler(_stack_frame: InterruptStackFrame) {
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    crate::task::keyboard::add_scancode(scancode);

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
