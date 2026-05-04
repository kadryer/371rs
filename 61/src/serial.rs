/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}

use uart_16550::SerialPort;

const SERIAL_IO_PORT: u16 = 0x3F8;

pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    let mut serial_port = unsafe { SerialPort::new(SERIAL_IO_PORT) };
    serial_port.write_fmt(args).unwrap();
}
