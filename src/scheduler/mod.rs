use alloc::BTreeMap;
use memory;
use spin::Mutex;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Process {
    pid: usize,
    //Memory allocated for this process.
    allocated_pages: usize,
    //Whether this hypothetical proces has been started or not.
    started: bool,
    start_pointer: usize,
    state: usize,
    //Stack pointer.
    stack: usize,
    trap_frame: usize,
    usage: usize,
}

impl Process {
    pub fn new(start_fn: usize, pid: usize, stack: usize) -> Self {
        Process {
            pid: pid,
            allocated_pages: 0,
            started: false,
            start_pointer: start_fn,
            state: 0,
            trap_frame: 0,
            stack: stack,
            usage: 0,
        }
    }
}

#[derive()]
pub struct Scheduler {
    //Currently running processes.
    procs: Mutex<BTreeMap<usize, Process>>,
    //Current processes.
    pub current: usize,
    //Number of processes.
    proc_count: usize,
    skip: usize,
}
