use device::pic::PICS;
use device::keyboard::ps2_keyboard::parse_key;
use device::ps2_8042::read_char;
use x86_64::structures::idt::ExceptionStackFrame;
use super::disable_interrupts_and_then;
use device::apic;

/// Timer handler checks the tick counter and if it exceeds 10, performs a round-robin context
/// switch to the next process.
pub extern "x86-interrupt" fn timer_handler(_stack_frame: &mut ExceptionStackFrame) {
    use core::sync::atomic::Ordering;
    use device::pit::PIT_TICKS;
    use task::{Scheduling, SCHEDULER};

    println!("timer interrupt.");

    apic::eoi();
    
    // Check if allocated timeslice finished (~20ms).
    if PIT_TICKS.fetch_add(1, Ordering::SeqCst) >= 10 {
        PIT_TICKS.store(0, Ordering::SeqCst);

        unsafe {
            // Call scheduler.
            disable_interrupts_and_then(|| {
                SCHEDULER.resched();
            });
        }
    }
}

pub extern "x86-interrupt" fn keyboard_handler(_stack_frame: &mut ExceptionStackFrame) {
    println!("keyboard interrupt.");
    let code = read_char();

    parse_key(code);
        
    apic::eoi();
}
