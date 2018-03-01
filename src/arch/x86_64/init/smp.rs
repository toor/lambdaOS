const AP_STACK_SIZE: usize = 4096;

pub struct ApCPU {
    id: usize,
    is_bsp: bool,
    stack: [u8; AP_STACK_SIZE],
}

pub extern "C" fn ap_start() -> ! {
    // TODO.
    unreachable!();
}
