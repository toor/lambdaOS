macro_rules! print {
    ($($arg:tt)*) => ({
        use device::serial;
        use core::fmt::Write;

        let _ = write!(serial::COM1.lock(), $($arg)*);
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

macro_rules! tty_switch {
    ($x:expr) => ({
        use device::vga::buffer::switch;

        switch($x);
    });
}
