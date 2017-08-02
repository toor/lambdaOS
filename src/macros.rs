macro_rules! get_current_pid {
    ($($arg:tt)*) => ({
        match $crate::memory_safe() {
            true => $crate::state().scheduler.current,
            false => 0,
        }
    });
}

macro_rules! kprint {
    ($fmt:expr) => ({
      print!(concat!("[{:?}] {:?} - ", $fmt, "\n"), get_current_pid!(), $crate::io::timer::real_time());
    });
    ($fmt:expr, $($arg:tt)*) => ({
      print!(concat!("[{:?}] {:?} - ", $fmt, "\n"), get_current_pid!(), $crate::io::timer::real_time(), $($arg)*);
    });
}

macro_rules! print {
   ($($arg:tt)*) => ({
      //$crate::io::drivers::display::text_buffer::print(format_args!($($arg)*));
      $crate::io::kprint::print(format_args!($($arg)*));
   });
}

macro_rules! format {
  ($($arg:tt)*) => ({
    use alloc::string::String;
    use core::fmt;
    let mut output = String::new();
    fmt::write(&mut output, format_args!($($arg)*));
    output
  });
}

macro_rules! debug {
  ($($arg:tt)*) => ({
    $crate::debug::debug();
  });
}