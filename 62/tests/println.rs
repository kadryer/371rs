


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

fn _check_println() {
    const MMIO: usize = 0xb8000;

    let rel: *mut u8 = MMIO as *mut u8;
    unsafe {
        kudos::serial_print!("Char: {:x}\n", *rel);
        assert!(*rel == b'R');
    }

    
    let rel: *mut u8 = (MMIO + 18*2) as *mut u8;
    unsafe {
        kudos::serial_print!("Char: {:x}\n", *rel);
        assert!(*rel == b'0');
    }

    let rel: *mut u8 = (MMIO + 160) as *mut u8;
    unsafe {
        kudos::serial_print!("Char: {:x}\n", *rel);
        assert!(*rel == b'R');
    }
    return
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    kudos::_test_runner(&[&_check_println]);
    kudos::qemu_quit(kudos::QEMU_FAIL);
    loop {}
}
