#![allow(static_mut_refs)]

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use x86_64::structures::idt::InterruptStackFrame;
use crate::print;
lazy_static::lazy_static! {
    pub static ref DIRECTION: spin::Mutex<Direction> = spin::Mutex::new(Direction::Right);
}
static mut IDT: x86_64::structures::idt::InterruptDescriptorTable =
    x86_64::structures::idt::InterruptDescriptorTable::new();
static mut CLOCK: usize = 0;
const TICK_RATE: usize = 10;


pub fn init_idt() {
    unsafe {
        IDT.breakpoint.set_handler_fn(breakpoint_handler);
        IDT.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(crate::gdt::DOUBLE_FAULT_IST_INDEX as u16);
        IDT[InterruptIndex::Timer as usize]
            .set_handler_fn(timer_handler);
        IDT[InterruptIndex::Keyboard as usize]
            .set_handler_fn(keyboard_interrupt_handler);
        IDT.page_fault.set_handler_fn(page_fault_handler);
        IDT.load();

        PICS.initialize();
        x86_64::instructions::interrupts::enable();
    }
}

pub fn set_direction(new_direction: Direction) {
    let mut direction = DIRECTION.lock();

    // Do nothing when attempting to 180
    match (*direction, new_direction) {
        (Direction::Up, Direction::Down) => {}
        (Direction::Down, Direction::Up) => {}
        (Direction::Left, Direction::Right) => {}
        (Direction::Right, Direction::Left) => {}
        _ => {
            *direction = new_direction;
        }
    }
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame,
) {
    crate::serial_println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    assert!(error_code == 0);
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_handler (
    _stack_frame: InterruptStackFrame,
) {
    unsafe {
        CLOCK += 1;
        if CLOCK % TICK_RATE == 0 { update() }
        PICS.notify_end_of_interrupt(InterruptIndex::Timer as u8)
    };
}

extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    lazy_static::lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(ScancodeSet1::new(),
                layouts::Us104Key, HandleControl::Ignore)
            );
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode('w') => set_direction(Direction::Up),
                DecodedKey::Unicode('a') => set_direction(Direction::Left),
                DecodedKey::Unicode('s') => set_direction(Direction::Down),
                DecodedKey::Unicode('d') => set_direction(Direction::Right),
                DecodedKey::Unicode('q') => crate::qemu_quit(crate::QEMU_PASS),
                DecodedKey::RawKey(_) => {},
                _ => {},
            }
        }
    }


    unsafe {
        PICS.notify_end_of_interrupt(InterruptIndex::Keyboard as u8);
    }
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: x86_64::structures::idt::PageFaultErrorCode,
) {
    crate::println!("EXCEPTION: PAGE FAULT");
    crate::println!("Accessed Address: {:?}", x86_64::registers::control::Cr2::read());
    crate::println!("Error Code: {:?}", error_code);
    crate::println!("{:#?}", stack_frame);
    crate::halt();
}

// PIC

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static mut PICS: pic8259::ChainedPics =
    unsafe { pic8259::ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) };

// update
fn update() {
    print!("{}", crate::snake::update_display())
}
