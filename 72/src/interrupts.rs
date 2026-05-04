#![allow(static_mut_refs)]

static mut IDT: x86_64::structures::idt::InterruptDescriptorTable =
    x86_64::structures::idt::InterruptDescriptorTable::new();

pub fn init_idt() {
    unsafe {
        IDT.breakpoint.set_handler_fn(breakpoint_handler);

        IDT.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(crate::gdt::DOUBLE_FAULT_IST_INDEX as u16);

        IDT.load();
    }
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: x86_64::structures::idt::InterruptStackFrame,
) {
    crate::serial_println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: x86_64::structures::idt::InterruptStackFrame,
    error_code: u64,
) -> ! {
    assert!(error_code == 0);
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
