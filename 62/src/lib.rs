#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(_test_runner)]

pub mod vga;
pub mod serial;

#[derive(Debug)]
pub enum QemuExitCode {
    Success = 0xA,
    Failed = 0xF,
}

pub const QEMU_PASS: QemuExitCode = QemuExitCode::Success;
pub const QEMU_FAIL: QemuExitCode = QemuExitCode::Failed;

pub fn qemu_quit(exit_code: QemuExitCode) {
    serial_print!("Exiting with code {:?}.\n", exit_code);
    unsafe {
        x86_64::instructions::port::Port::new(0xf4).write(exit_code as u32);
    }
}

pub fn test_panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
        serial_print!("panic occurred in file '{}' at line {}.\n",
            location.file(),
            location.line(),
        );
    } else {
        serial_println!("panic occurred but can't get location information...\n");
    }

    qemu_quit(QEMU_FAIL);
    loop {}
}

pub fn _test_runner(tests: &[&dyn Fn()]) {
    for i in 0..tests.len() {
        print!("Running test case {:0x}... ", i);
        serial_print!("Running test case {:0x}...\n", i);

        tests[i]();

        println!("Success.");
        serial_print!("Success.\n");
    }
}

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    _test_runner(&[&_ex]);
    qemu_quit(QEMU_PASS);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test_panic(info)
}

fn _ex() {
    assert!(true);
    return
}


