#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points

mod game_of_life;
mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.
pub extern "C" fn _start() -> ! {
    game_of_life::run();
}

/// This function is called on panic.
#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    print!("{}", panic_info);
    loop {}
}
