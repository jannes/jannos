use core::fmt::{self, Write};

use crate::sbi::{sbi_out, CONSOLE};

// Normal printing to console with mutual exclusion
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    CONSOLE.lock().write_fmt(args).unwrap();
}

// Printing to console without mutual exclusion for panics
#[doc(hidden)]
pub fn _print_panic(args: fmt::Arguments) {
    sbi_out().write_fmt(args).unwrap();
}

// Macros copied from octox
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::print::_print(format_args!($($arg)*))
    };
}
#[macro_export]
macro_rules! println {
    ($fmt:literal$(, $($arg: tt)+)?) => {
        $crate::print::_print(format_args!(concat!($fmt, "\n") $(,$($arg)+)?))
    };
}

#[macro_export]
macro_rules! println_panic {
    ($fmt:literal$(, $($arg: tt)+)?) => {
        $crate::print::_print_panic(format_args!(concat!($fmt, "\n") $(,$($arg)+)?))
    };
}
