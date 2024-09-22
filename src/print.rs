use core::fmt::{self, Write};

use crate::sbi::{self};

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    sbi::sbi_out().write_fmt(args).unwrap();
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
