#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kudos::_test_runner)]
#![reexport_test_harness_main = "test_main"]

use kudos::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start(boot_info: &'static bootloader::BootInfo) -> ! {
    println!("{:?}", boot_info);
    kudos::init(boot_info);

    #[cfg(test)]
    {
        test_main();
        kudos::qemu_quit(kudos::QEMU_PASS);
    }
    println!("It did not crash!");

    println!("Level 4 page table at: {:?}", x86_64::registers::control::Cr3::read().0.start_address());


    // Address manipulation
    let offset = x86_64::VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { kudos::memory::init(offset) };
    let mut frame_allocator = unsafe { kudos::memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    
    // map an unused page
    let page = x86_64::structures::paging::Page::containing_address(x86_64::VirtAddr::new(0));
    kudos::memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { ptr.write_volatile(0x_f021_f077_f065_f04e) };

    println!("FIN");

    kudos::allocator::init_heap(&mut mapper, &mut frame_allocator);

    println!("Hello world{}", "!");

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
