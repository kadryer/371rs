#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kudos::_test_runner)]

mod vga;
mod serial;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("I'm main.");

    #[cfg(test)]
    serial_println!("Begin Test");
    unsafe {
        x86_64::instructions::port::Port::new(0xf4).write(0xAu32);
    }

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn test_panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("PANIC: {}", info);
    loop {}
}
