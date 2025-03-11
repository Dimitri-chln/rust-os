use x86_64::registers::control::Cr2;
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

use crate::{println_vga, utils};

pub extern "x86-interrupt" fn handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    println_vga!("EXCEPTION: PAGE FAULT");
    println_vga!("Accessed Address: {:?}", Cr2::read());
    println_vga!("Error Code: {:?}", error_code);
    println_vga!("{:#?}", stack_frame);
    utils::hlt_loop();
}
