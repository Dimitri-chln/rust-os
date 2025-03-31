use bootloader_api::info::{FrameBuffer, Optional};
use drivers::display::frame_buffer;

use crate::{gdt, interrupts};

pub fn init(frame_buffer: &mut Optional<FrameBuffer>) {
    gdt::init();
    interrupts::idt::init();
    interrupts::pics::init();
    x86_64::instructions::interrupts::enable();

    if let Some(frame_buffer) = frame_buffer.take() {
        frame_buffer::init(frame_buffer);
    }
}
