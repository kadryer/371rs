#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(_test_runner)]

pub mod vga;
pub mod serial;

const _EX: [fn(); 2] = [_hi, _bye];


pub fn _test_runner(_tests: &[&dyn Fn()]) {
    let fs = [_EX, _EX, _EX];
    for i in 0..fs.len() {
        serial_print!("Beginning Test Loop 0x{:02x}... \n", i);
        for j in fs[i] {
            serial_print!(" Beginning test 0x{:02x}... ", i);
            j();
            serial_println!("  [Pass]");
        }
    }
    _test_quit();
}


#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("TEST PANIC: {}", info);
    loop {}
}

fn _test_quit() {
    unsafe { x86_64::instructions::port::Port::new(0xf4).write(0xAu32) };
}

fn _hi() {
    println!("Hello world!");
    return;
}

fn _bye() {
    println!("Goodbye space!");
    return;
}
