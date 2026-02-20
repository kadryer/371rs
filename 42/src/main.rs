#![no_std]
#![no_main]
#![allow(unconditional_recursion)]

const VGA_AD: *mut u8 = 0xb8000 as *mut u8;
const S: &str = "Hello World!";
const COL: u8 = 0xE;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> () {
    print_vga(S, COL);
}

pub fn print_vga(s: &str, color_code: u8) {
    if s.len() <= 24 {
        let mut offset: usize = 0;
        for byte in s.bytes() {
            unsafe {
                *VGA_AD.add(offset) = byte;
                *VGA_AD.add(offset + 1) = color_code;
            }
            offset += 2;
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
