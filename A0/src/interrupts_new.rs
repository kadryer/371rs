#![allow(static_mut_refs)]

#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
}

static mut IDT: x86_64::structures::idt::InterruptDescriptorTable =
    x86_64::structures::idt::InterruptDescriptorTable::new();

pub fn init_idt() {
    unsafe {
        IDT.breakpoint.set_handler_fn(breakpoint_handler);

        IDT.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(crate::gdt::DOUBLE_FAULT_IST_INDEX as u16);

        IDT[InterruptIndex::Timer as usize].set_handler_fn(timer_handler);

        IDT.load();
        PICS.initialize();
        PICS.write_masks(0xFE, 0xFF);
        x86_64::instructions::interrupts::enable();
    }
}

extern "x86-interrupt" fn breakpoint_handler(
    _stack_frame: x86_64::structures::idt::InterruptStackFrame,
) {
    crate::serial_println!("BREAKPOINT HANDLER HIT");
    crate::qemu_quit(crate::QEMU_PASS);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: x86_64::structures::idt::InterruptStackFrame,
    error_code: u64,
) -> ! {
    assert!(error_code == 0);
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_handler(
    _stack_frame: x86_64::structures::idt::InterruptStackFrame,
) {
    crate::println!("INTERRUPT: TIMER");
    crate::serial_println!("INTERRUPT: TIMER");
    unsafe { PICS.notify_end_of_interrupt(InterruptIndex::Timer as u8) };
}

// PIC

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static mut PICS: pic8259::ChainedPics =
    unsafe { pic8259::ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) };
