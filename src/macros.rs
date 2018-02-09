macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::device::vga::buffer::print(format_args!($($arg)*));
    });
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! format {
    ($($arg:tt)*) => ({
        use alloc::string::String;
        use core::fmt;
        let mut output = String::new();
        fmt::write(&mut output, format_args!($($arg)*)).unwrap();
        output
    });
}
