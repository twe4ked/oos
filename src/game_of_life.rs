use crate::vga_buffer::{BUFFER_HEIGHT, BUFFER_WIDTH};

pub const WORLD_WIDTH: usize = BUFFER_WIDTH / 2;
pub const WORLD_HEIGHT: usize = BUFFER_HEIGHT;

type Cells = [[bool; WORLD_WIDTH]; WORLD_HEIGHT];

#[rustfmt::skip]
const OFFSETS: [(i8, i8); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),/* 0  0 */( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

pub fn simulate(mut cells: Cells) -> Cells {
    let old_cells = cells.clone();

    for y in 0..WORLD_HEIGHT {
        for x in 0..WORLD_WIDTH {
            let cell = old_cells[y][x];

            let mut neighbors = 0;
            for (x_offset, y_offset) in &OFFSETS {
                let x = add_offset(WORLD_WIDTH - 1, x, *x_offset);
                let y = add_offset(WORLD_HEIGHT - 1, y, *y_offset);

                if old_cells[y][x] {
                    neighbors += 1;
                }
            }

            if cell && (neighbors < 2 || neighbors > 3) {
                cells[y][x] = false;
            } else if !cell && neighbors == 3 {
                cells[y][x] = true;
            }
        }
    }

    cells
}

fn add_offset(max: usize, n: usize, offset: i8) -> usize {
    let min = 0;

    match n as isize + offset as isize {
        r if r > max as isize => min,
        r if r < min as isize => max,
        r => r as usize,
    }
}
