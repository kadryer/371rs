#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(_test_runner)]
#![feature(abi_x86_interrupt)]

extern crate alloc;

pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod vga;
pub mod memory;
pub mod allocator;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
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

pub fn init(_boot_info: &'static bootloader::BootInfo) {
    gdt::init_gdt();
    interrupts::init_idt();
    
    // Heap Init
    let offset = x86_64::VirtAddr::new(_boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(offset) };
    let mut frame_allocator = unsafe { memory::BootInfoFrameAllocator::init(&_boot_info.memory_map) };
    //let page = x86_64::structures::paging::Page::containing_address(x86_64::VirtAddr::new(0));
    //memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    //let ptr: *mut u64 = page.start_address().as_mut_ptr();
    //unsafe { ptr.write_volatile(0x_f021_f077_f065_f04e) };

    println!("FIN");

    allocator::init_heap(&mut mapper, &mut frame_allocator).unwrap();
}

pub fn halt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn test_panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
        serial_print!(
            "panic occurred in file '{}' at line {}.\n",
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
        serial_print!("Running test case {:0x}...\n", i);
        tests[i]();
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
}
