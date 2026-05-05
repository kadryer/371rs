#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kudos::_test_runner)]

#[panic_handler]
fn test_panic(_info: &core::panic::PanicInfo) -> ! {
    kudos::serial_println!("[Pass]");
    kudos::qemu_quit(kudos::QEMU_PASS);
    loop {}
}

fn bad() {
    assert!(false);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    kudos::_test_runner(&[&bad]);
    kudos::qemu_quit(kudos::QEMU_FAIL);
    loop {}
}
