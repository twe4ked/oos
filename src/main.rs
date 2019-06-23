#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points

mod game_of_life;
mod vga_buffer;

use core::panic::PanicInfo;
use game_of_life::{WORLD_HEIGHT, WORLD_WIDTH};
use vga_buffer::{Color, BUFFER_HEIGHT, BUFFER_WIDTH};

#[no_mangle]
/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.
pub extern "C" fn _start() -> ! {
    let mut cells = [[false; WORLD_WIDTH]; WORLD_HEIGHT];

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
            let mut i = 0;
            for x in 0..BUFFER_WIDTH {
                if x % 2 != 0 {
                    i += 1;
                    continue;
                }

                let color = if cells[y][i] {
                    Color::LightCyan
                } else {
                    Color::Black
                };

                vga_buffer::WRITER
                    .lock()
                    .write_char(' ' as u8, x, y, color, color);
                vga_buffer::WRITER
                    .lock()
                    .write_char(' ' as u8, x + 1, y, color, color);
            }
        }

        cells = game_of_life::simulate(cells);
    }
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
            .write_char(*b, i, 0, Color::Black, Color::Yellow);
    }
    loop {}
}
