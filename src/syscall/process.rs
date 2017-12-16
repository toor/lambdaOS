use alloc::String;
use task::{Scheduling, ProcessId, SCHEDULER};

//Simple system call that wraps creating a process and marking it as ready.
pub fn create(new: extern "C" fn(), name: String) -> ProcessId {
    let pid = SCHEDULER
        .create(new, name)
        .expect("Failed to create new process");
    SCHEDULER.ready(pid.clone());
    pid
}
