#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kudos::_test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga;
mod serial;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("I'm main{}", ".");
    serial_println!("I'm main{}", ".");

    #[cfg(test)]
    test_main();

    #[cfg(test)]
    kudos::qemu_quit(kudos::QEMU_PASS);

    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
