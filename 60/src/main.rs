#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

mod vga;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("I'm main{}", ".");

    #[cfg(test)]
    test_runner(&[]);

    loop {}
}

#[cfg(test)]
fn test_runner(_tests: &[&dyn Fn()]) {
    let fs = [_ex];
    for i in 0..fs.len() {
        println!("Running test case {:0x}", i);
        fs[i]();
        println!("Success.");
    }

    // Exit Status 21 (Success)
    unsafe {
        x86_64::instructions::port::Port::new(0xf4).write(0xAu32);
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn _ex() {
    assert!(true);
    return
}
