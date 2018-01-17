use alloc::string::String;
use alloc::vec::Vec;
use task::context::Context;

#[derive(Clone, Debug, Eq, PartialEq)]
/// Current state of the process.
pub enum State {
    /// Process is free.
    Free,
    /// Process is the current process.
    Current,
    /// Process has been stopped.
    Suspended,
    /// Process is ready to be ran by the scheduler.
    Ready,
}

#[derive(Clone, Debug)]
/// Process priority.
pub struct Priority(pub u64);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
/// Tuple type for PID.
pub struct ProcessId(pub usize);

impl ProcessId {
    /// Null kernel process.
    pub const NULL_PROC: ProcessId = ProcessId(0);

    pub fn inner(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Debug)]
/// A single process on the system.
/// It has register context, id, name and an Optional process stack.
pub struct Process {
    pub pid: ProcessId,
    pub name: String,
    pub state: State,
    pub priority: Priority,
    pub ctx: Context,
    pub stack: Option<Vec<usize>>,
}

impl Process {
    pub fn new(id: ProcessId) -> Self {
        Process {
            pid: id,
            name: String::from("new_proc"),
            state: State::Suspended,
            priority: Priority(0),
            ctx: Context::new(),
            stack: None,
        }
    }
    
    /// Set the state of the process.
    pub fn set_state(&mut self, new: State) {
        self.state = new;
    }
    
    /// Set `cr3` to point to the address specified by `addr`.
    pub fn set_page_table(&mut self, addr: usize) {
        self.ctx.set_page_table(addr);
    }
    
    /// Set the stack pointer register.
    pub fn set_stack(&mut self, addr: usize) {
        self.ctx.set_stack(addr);
    }
}

///A returned process pops an instruction pointer off the stack then jumps to it.
/// The IP from the stack will point to this function.
#[naked]
pub unsafe extern "C" fn process_return() {
    use task::Scheduling;
    use alloc::boxed::Box;

    // Pop a pointer to the self object off the stack.
    let scheduler_ptr: *mut &Scheduling;
    asm!("pop $0" : "=r"(scheduler_ptr) : : "memory" : "intel", "volatile");

    let scheduler = Box::from_raw(scheduler_ptr);

    let current: ProcessId = scheduler.get_id();
    
    // Process returned, we kill it
    scheduler.kill(current);
}
