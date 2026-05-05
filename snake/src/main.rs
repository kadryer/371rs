#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kudos::_test_runner)]
#![reexport_test_harness_main = "test_main"]

use kudos::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start(boot_info: &'static bootloader::BootInfo) -> ! {
    // Initialize Environment
    kudos::init(boot_info);

    // Test Environment
    #[cfg(test)]
    {
        test_main();
        kudos::qemu_quit(kudos::QEMU_PASS);
    }

    kudos::snake::snake_game();

    kudos::halt();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    kudos::halt();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    kudos::test_panic(info);
}
