#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points

static HELLO: &[u8] = b"Hello World!";

mod vga_buffer;

use core::panic::PanicInfo;
use vga_buffer::Color;

#[no_mangle]
/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.
pub extern "C" fn _start() -> ! {
    for (i, &byte) in HELLO.iter().enumerate() {
        vga_buffer::WRITER
            .lock()
            .write_char(byte, i, 0, Color::Black, Color::Yellow);
    }

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
