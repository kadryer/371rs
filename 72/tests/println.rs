#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kudos::_test_runner)]

#[panic_handler]
fn test_panic(info: &core::panic::PanicInfo) -> ! {
    kudos::test_panic(info)
}

fn _check_println() {
    const MMIO: usize = 0xb8000;

    kudos::println!("1");
    kudos::serial_println!("1\n");
    kudos::println!("2");
    kudos::serial_println!("2\n");

    let rel: *mut u8 = MMIO as *mut u8;
    unsafe {
        kudos::serial_print!("Char: {:x}\n", *rel);
        assert!(*rel == b'1');
    }

    let rel: *mut u8 = (MMIO + 80 * 2) as *mut u8;
    unsafe {
        kudos::serial_print!("Char: {:x}\n", *rel);
        assert!(*rel == b'2');
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    kudos::_test_runner(&[&_check_println]);
    kudos::qemu_quit(kudos::QEMU_PASS);
    loop {}
}
