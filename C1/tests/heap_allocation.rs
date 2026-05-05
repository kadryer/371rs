#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kudos::_test_runner)]

extern crate alloc;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    kudos::test_panic(info)
}

fn simple_allocation() {
    let heap_value_1 = alloc::boxed::Box::new(41);
    kudos::serial_println!("test");
    let heap_value_2 = alloc::boxed::Box::new(13);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 13);
}

fn large_vec() {
    let n = 1000;
    let mut vec = alloc::vec::Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

fn many_boxes() {
    for i in 0..kudos::allocator::HEAP_SIZE {
        let x = alloc::boxed::Box::new(i);
        assert_eq!(*x, i);
    }
}



#[unsafe(no_mangle)]
pub extern "C" fn _start(boot_info: &'static bootloader::BootInfo) -> ! {
    kudos::init(boot_info);
    kudos::_test_runner(&[&simple_allocation, &large_vec, &many_boxes]);
    kudos::qemu_quit(kudos::QEMU_PASS);
    kudos::halt();
}
