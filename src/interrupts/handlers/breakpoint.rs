use x86_64::structures::idt::InterruptStackFrame;

use crate::println_vga;

pub extern "x86-interrupt" fn handler(stack_frame: InterruptStackFrame) {
    println_vga!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
