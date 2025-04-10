use core::fmt::Write;

use x86_64::instructions::interrupts;

use crate::println_vga;

use super::WRITER;
use super::constants::BUFFER_HEIGHT;

#[test_case]
fn test_println_simple() {
    println_vga!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println_vga!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");

        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}
