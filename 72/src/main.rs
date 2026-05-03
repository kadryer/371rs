#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

mod vga;
mod serial;

const _EX: [fn(); 2] = [_hi, _bye];

#[cfg(test)]
fn test_runner(_tests: &[&dyn Fn()]) {
    let fs = [_EX, _EX, _EX];
    for i in 0..fs.len() {
        serial_print!("Beginning Test Loop 0x{:02x}... \n", i);
        for j in fs[i] {
            serial_print!(" Beginning test 0x{:02x}... ", i);
            j();
            serial_println!("  [Pass]");
        }
        
    }
    unsafe { x86_64::instructions::port::Port::new(0xf4).write(0xAu32) };
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("I'm main.");

    #[cfg(test)]
    test_runner(&[]);
    unsafe {
        x86_64::instructions::port::Port::new(0xf4).write(0xAu32);
    }

    loop {}
}

fn _hi() {
    println!("Hello world!");
    return;
}

fn _bye() {
    println!("Goodbye space!");
    return;
}
