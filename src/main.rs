#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points

mod game_of_life;
mod vga_buffer;

use core::panic::PanicInfo;
use vga_buffer::{Color, BUFFER_HEIGHT};

#[no_mangle]
/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.
pub extern "C" fn _start() -> ! {
    game_of_life::run();
}

/// This function is called on panic.
#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    let message = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
        s
    } else {
        "panic [no message]"
    };
    let bytes = message.as_bytes().iter().enumerate();
    for (i, b) in bytes {
        vga_buffer::WRITER
            .lock()
            .write_char(*b, i, BUFFER_HEIGHT - 1, Color::Black, Color::Yellow);
    }
    loop {}
}
