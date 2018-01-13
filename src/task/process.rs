use alloc::string::String;
use alloc::vec::Vec;
use task::context::Context;

#[derive(Clone, Debug, Eq, PartialEq)]
///Current state of the process.
pub enum State {
    Free,
    Current,
    Suspended,
    Ready,
}

#[derive(Clone, Debug)]
///Process priority.
pub struct Priority(pub u64);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
///Tuple type for PID.
pub struct ProcessId(pub usize);

impl ProcessId {
    pub const NULL_PROC: ProcessId = ProcessId(0);

    pub fn inner(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Debug)]
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

    pub fn set_state(&mut self, new: State) {
        self.state = new;
    }

    pub fn set_page_table(&mut self, addr: usize) {
        self.ctx.set_page_table(addr);
    }

    pub fn set_stack(&mut self, addr: usize) {
        self.ctx.set_stack(addr);
    }
}

//A returned process pops an instruction pointer off the stack then jumps to it.
//The IP from the stack will point to this function.
#[naked]
pub unsafe extern "C" fn process_return() {
    use task::Scheduling;
    use alloc::boxed::Box;

    // Pop a pointer to the self object off the stack.
    let scheduler_ptr: *mut &Scheduling;
    asm!("pop $0" : "=r"(scheduler_ptr) : : "memory" : "intel", "volatile");

    let scheduler = Box::from_raw(scheduler_ptr);

    let current: ProcessId = scheduler.get_id();
    //Process returned, we kill it
    scheduler.kill(current);
}
