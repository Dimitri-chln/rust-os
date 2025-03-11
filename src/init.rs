use bootloader::BootInfo;
use x86_64::VirtAddr;

use crate::{
    allocator, gdt, interrupts,
    memory::{self, frame_allocator::BootInfoFrameAllocator},
};

pub fn init(boot_info: &'static BootInfo) {
    gdt::init();
    interrupts::idt::init();
    interrupts::pics::init();
    x86_64::instructions::interrupts::enable();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
}
