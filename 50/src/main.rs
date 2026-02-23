#![no_std]
#![no_main]

mod vga;

#[panic_handler]
#[allow(unconditional_recursion)]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic(info)
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    vga::str_to_vga("Hello, world!");
    loop {}
}