#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points

mod game_of_life;
mod vga_buffer;

use core::panic::PanicInfo;
use vga_buffer::{Color, BUFFER_HEIGHT, BUFFER_WIDTH};

#[no_mangle]
/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.
pub extern "C" fn _start() -> ! {
    let mut cells = [[false; BUFFER_WIDTH]; BUFFER_HEIGHT];

    // Glider:
    //
    // - 0 1 2 3
    // 0 - - # -
    // 1 # - # -
    // 2 - # # -
    // 3 - - - -
    cells[0][2] = true;
    cells[1][0] = true;
    cells[1][2] = true;
    cells[2][1] = true;
    cells[2][2] = true;

    loop {
        for y in 0..BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH {
                let color = if cells[y][x] {
                    Color::LightCyan
                } else {
                    Color::Black
                };

                vga_buffer::WRITER
                    .lock()
                    .write_char(' ' as u8, x, y, color, color);
            }
        }

        cells = game_of_life::simulate(cells);
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
