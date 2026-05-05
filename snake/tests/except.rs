#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kudos::_test_runner)]

#[panic_handler]
fn test_panic(info: &core::panic::PanicInfo) -> ! {
    kudos::serial_print!("{}", info);
    kudos::serial_println!("[Pass]");
    kudos::qemu_quit(kudos::QEMU_PASS);
    loop {}
}

fn except() {
    x86_64::instructions::interrupts::int3();
}

#[unsafe(no_mangle)]
pub extern "C" fn _start(boot_info: &'static bootloader::BootInfo) -> ! {
    kudos::init(boot_info);
    kudos::_test_runner(&[&except]);
    kudos::qemu_quit(kudos::QEMU_PASS);
    loop {}
}
