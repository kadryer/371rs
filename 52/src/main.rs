#![no_std]
#![no_main]

mod colors;
mod img;
mod vga;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    //colors::colors();
    colors::image();
    loop {}
}
