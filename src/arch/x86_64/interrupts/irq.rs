use device::pic::PICS;
use device::keyboard::ps2_keyboard::parse_key;
use device::ps2_8042::read_char;
use x86_64::structures::idt::ExceptionStackFrame;
use super::disable_interrupts_and_then;

/// Timer handler checks the tick counter and if it exceeds 10, performs a round-robin context
/// switch to the next process.
pub extern "x86-interrupt" fn timer_handler(_stack_frame: &mut ExceptionStackFrame) {
    use core::sync::atomic::Ordering;
    use device::pit::PIT_TICKS;
    use task::{SCHEDULER, Scheduling};

    unsafe { PICS.lock().notify_end_of_interrupt(0x20) };

    if PIT_TICKS.fetch_add(1, Ordering::SeqCst) >= 10 {
        PIT_TICKS.store(0, Ordering::SeqCst);

        unsafe {
            disable_interrupts_and_then(|| {
                    SCHEDULER.resched();
            });
        }
    }
}

pub extern "x86-interrupt" fn keyboard_handler(_stack_frame: &mut ExceptionStackFrame) {
    disable_interrupts_and_then(|| {
        let code = read_char();

        parse_key(code);

        unsafe { PICS.lock().notify_end_of_interrupt(0x21) };
    });
}
