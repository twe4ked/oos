#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.
pub extern "C" fn _start() -> ! {
    vga_buffer::WRITER.lock().write_str("Hello again");

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
