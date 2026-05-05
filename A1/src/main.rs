#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kudos::_test_runner)]
#![reexport_test_harness_main = "test_main"]

use kudos::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    

    kudos::init();

    #[cfg(test)]
    {
        test_main();
        kudos::qemu_quit(kudos::QEMU_PASS);
    }
    println!("It did not crash!");

    println!("Level 4 page table at: {:?}", x86_64::registers::control::Cr3::read().0.start_address());

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
