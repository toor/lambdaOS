use core::fmt;

#[derive(Copy, Clone)]
pub struct Time {
    //seconds
    pub secs: u64,
    //milliseconds
    pub millis: u32,
    //nanoseconds
    pub nanos: u32,
}

impl Time {
    //New time object
    pub fn new(secs: u64, millis: u32, nanos: u32) -> Self {
        Time {
            secs: secs,
            millis: millis,
            nanos: nanos,
        }
    }
}

impl fmt::Debug for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.secs)
    }
}