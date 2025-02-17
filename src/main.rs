#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use rust_os::{allocator, memory, println};
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    rust_os::init();

    println!("Hello World!");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // new
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );

    rust_os::hlt_loop();
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    rust_os::panic::handler(info)
}
