#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader_api::config::{BootloaderConfig, Mapping};
use bootloader_api::{entry_point, BootInfo};
use drivers::fs::ext2::Ext2;
use drivers::fs::traits::FileSystem;
use drivers::println;
use kernel::{allocator, memory};
use utils::hlt::hlt_loop;
use utils::posix::path::PathBuf;
use x86_64::VirtAddr;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    kernel::init(&mut boot_info.framebuffer);

    println!("Hello World!");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_regions) };

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

    // let fs = unsafe { Ext2::new(VirtAddr::new(0)).unwrap() };
    // let inode = fs.read(PathBuf::from("/home/dimitri"), None);
    // println!("{inode:?}");

    hlt_loop();
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    kernel::panic::handler(info)
}
