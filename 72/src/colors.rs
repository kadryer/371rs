-const COLS: usize = 80;
const ROWS: usize = 25;
const MMIO: usize = 0xb8000;
const NUM_COLORS: usize = 16;
const COL_WIDTH: usize = COLS / NUM_COLORS;

use crate::img;

#[unsafe(no_mangle)]
pub fn colors() {
    let mut latest: usize = 0;
    for _ in 0..ROWS {
        let mut input: u16 = 0x0000;
        for col in 0..COLS {
            if (col != 0) && (col % COL_WIDTH == 0) {
                input += 0x1000;
            }
            unsafe {
                let rel: *mut u16 = (MMIO + (latest * 2)) as *mut u16;
                *rel = input;
            }
            latest += 1;
        }
    }
}

pub fn image() {
    let mut latest: usize = 0;
    let vga_buffer: [[u16; 80]; 25] = img::VGA_BUFFER;
    for row in vga_buffer {
        for pixel in row {
            unsafe {
                let rel: *mut u16 = (MMIO + (latest * 2)) as *mut u16;
                *rel = pixel
            }
            latest += 1
        }
    }
}
