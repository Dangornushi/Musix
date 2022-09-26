#![no_std]
pub mod ascii;
pub mod console;
pub mod graphics;
pub mod pci;

use crate::console::Console;
use core::arch::asm;
use core::fmt::Write;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_put(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn _put(args: core::fmt::Arguments) {
    let console = Console::instance();
    console.write_fmt(args).unwrap();
}
