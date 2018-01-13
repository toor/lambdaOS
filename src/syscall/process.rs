use alloc::String;
use task::{ProcessId, Scheduling, SCHEDULER};
use arch::interrupts::disable_interrupts_and_then;

//Simple system call that wraps creating a process and marking it as ready.
pub fn create(new: extern "C" fn(), name: String) -> ProcessId {
    disable_interrupts_and_then(|| -> ProcessId {
        let pid = SCHEDULER
            .create(new, name)
            .expect("Could not create new process!");
        SCHEDULER.ready(pid.clone());
        pid
    })
}
