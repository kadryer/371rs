#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

mod vga;
mod serial;

#[derive(Debug)]
pub enum QemuExitCode {
    Success = 0xA,
    Failed = 0xF,
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("I'm main{}", ".");

    #[cfg(test)]
    test_runner(&[]);

    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[cfg(test)]
fn test_runner(_tests: &[&dyn Fn()]) {
    let fs = [_ex];
    for i in 0..fs.len() {
        print!("Running test case {:0x}... ", i);
        serial_print!("Running test case {:0x}...\n", i);
        fs[i]();
        println!("Success.");
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
        serial_print!("panic occurred in file '{}' at line {}.\n",
            location.file(),
            location.line(),
        );
    } else {
        serial_println!("panic occurred but can't get location information...\n");
    }
    exit_qemu(QemuExitCode::Failed);
    loop {}
}


fn _ex() {
    assert!(false);
    return
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    serial_print!("Exiting with code {:?}.\n", exit_code);
    unsafe {
        x86_64::instructions::port::Port::new(0xf4).write(exit_code as u32);
    }
}
