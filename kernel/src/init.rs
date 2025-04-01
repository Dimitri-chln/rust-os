use bootloader_api::info::BootInfo;
use drivers::display::frame_buffer;
use x86_64::VirtAddr;

use crate::{gdt, heap, interrupts, memory};

pub fn init(boot_info: &'static mut BootInfo) {
    gdt::init();
    interrupts::idt::init();
    interrupts::pics::init();
    x86_64::instructions::interrupts::enable();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_regions) };

    heap::init(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    if let Some(frame_buffer) = boot_info.framebuffer.take() {
        frame_buffer::init(frame_buffer);
    }
}
