#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kudos::_test_runner)]
#![reexport_test_harness_main = "test_main"]

use kudos::println;
use x86_64::structures::paging::Translate;

#[unsafe(no_mangle)]
pub extern "C" fn _start(boot_info: &'static bootloader::BootInfo) -> ! {
    println!("{:?}", boot_info);
    kudos::init();

    #[cfg(test)]
    {
        test_main();
        kudos::qemu_quit(kudos::QEMU_PASS);
    }
    println!("It did not crash!");

    println!("Level 4 page table at: {:?}", x86_64::registers::control::Cr3::read().0.start_address());


    // Address Lookup
    let addresses = [0x1000, 0x2000, 0xff00];
    let offset = x86_64::VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { kudos::memory::init(offset) };
    for &address in &addresses {
        let virt = x86_64::VirtAddr::new(address);
        let phys = mapper.translate(virt);
        println!("{:?} -> {:?}", virt, phys);
}

    println!("FIN");

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
