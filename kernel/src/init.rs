use crate::{gdt, interrupts};

pub fn init() {
    gdt::init();
    interrupts::idt::init();
    interrupts::pics::init();
    x86_64::instructions::interrupts::enable();
}
