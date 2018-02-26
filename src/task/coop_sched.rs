use alloc::VecDeque;
use alloc::vec::Vec;
use alloc::String;
use core::mem;
use core::ops::DerefMut;
use core::sync::atomic::{AtomicUsize, Ordering};
use task::{Process, ProcessId, ProcessList, Scheduling, State, INITIAL_STACK};
use task::process;
use spin::RwLock;

/// Global kernel scheduler type.
pub type Scheduler = CoopScheduler;

/// A simple cooperative scheduler. It uses round-robin scheduling, where the next available, ready
/// process is the next process to be ran.
pub struct CoopScheduler {
    current_pid: AtomicUsize,    
    task_table: RwLock<ProcessList>,
    ready_list: RwLock<VecDeque<ProcessId>>,
}

impl Scheduling for CoopScheduler {
    /// Create a process using a C-declared function pointer as an argument. This function allocates a
    /// 1 KiB stack.
    fn create(&self, func: extern "C" fn(), name: String) -> Result<ProcessId, i16> {
        use arch::memory::paging;

        let mut stack: Vec<usize> = vec![0; INITIAL_STACK];

        let proc_top: usize = stack.len() - 3;

        let proc_sp = stack.as_ptr() as usize + (proc_top * mem::size_of::<usize>());

        use alloc::boxed::Box;
        let self_ptr: Box<&Scheduling> = Box::new(self);
        
        // Reserve three elements on the stack.
        // stack.len() - 3 -> pointer to the entry point of the process. This is what RSP is set to
        // under Context::switch_to().
        // stack.len() - 2 -> function that we jump to after process return.

        let stack_vals: Vec<usize> = vec![
            func as usize,
            process::process_return as usize,
            Box::into_raw(self_ptr) as usize,
        ];
        
        // Properly reserve these blocks on the stack.
        for (i, val) in stack_vals.iter().enumerate() {
            stack[proc_top + i] = *val;
        }

        let mut task_table_lock = self.task_table.write();

        let proc_lock = task_table_lock.add()?;
        {
            let mut process = proc_lock.write();

            process.stack = Some(stack);
            process.name = name;
            
            // Create a new page table. This saves the address placed in cr3 after page table
            // creation for a context switch later on.
            process
                .ctx
                .set_page_table(unsafe { paging::ActivePageTable::new().address() });
            
            // Set the stack pointer.
            process.ctx.set_stack(proc_sp);

            Ok(process.pid)
        }
    }
    
    /// Returns the PID of the current process.
    fn get_id(&self) -> ProcessId {
        ProcessId(self.current_pid.load(Ordering::SeqCst))
    }
    
    /// Kill the process. We do this by marking it as free in the task table.
    /// To free memory held by the process, we drop the String that holds the process name,
    /// and mark the Option stack as None - this causes the memory held by the Some() to be
    /// dropped.
    fn kill(&self, id: ProcessId) {
        {
            let task_table_lock = self.task_table.read();
            let mut proc_lock = task_table_lock
                .get(id)
                .expect("Cannot kill a non-existent process")
                .write();

            proc_lock.set_state(State::Free);
            proc_lock.stack = None;
            drop(&mut proc_lock.name);
        }

        unsafe {
            self.resched();
        }
    }
    
    /// Mark a process as ready which enables it to be ran under resched().
    fn ready(&self, id: ProcessId) {
        self.ready_list.write().push_back(id);
    }
    
    /// Perform a context switch to the new process. This method will deadlock if any software
    /// locks are still held - it is therefore important to scope locking of data structures to
    /// ensure that these locks will be dropped.
    unsafe fn resched(&self) {
        {
            if self.ready_list.read().is_empty() {
                return;
            }
        }

        let mut prev_ptr = 0 as *mut Process;
        let mut next_ptr = 0 as *mut Process;

        // Separate the locks from the context switch through scoping
        {
            let task_table_lock = self.task_table.read();
            let mut ready_list_lock = self.ready_list.write();

            let curr_id: ProcessId = self.get_id();

            let mut prev = task_table_lock
                .get(curr_id)
                .expect("Could not find old process")
                .write();

            if prev.state == State::Current {
                prev.set_state(State::Ready);
                ready_list_lock.push_back(curr_id);
            }

            if let Some(next_id) = ready_list_lock.pop_front() {
                if next_id != self.get_id() {
                    let mut next = task_table_lock
                        .get(next_id)
                        .expect("Could not find new process")
                        .write();

                    next.set_state(State::Current);

                    self.current_pid.store(next.pid.inner(), Ordering::SeqCst);

                    // Save process pointers for out of scope context switch
                    prev_ptr = prev.deref_mut() as *mut Process;
                    next_ptr = next.deref_mut() as *mut Process;
                }
            }
        }

        if next_ptr as usize != 0 {
            assert!(
                prev_ptr as usize != 0,
                "Pointer to new proc has not been set!"
            );

            let prev: &mut Process = &mut *prev_ptr;
            let next: &mut Process = &mut *next_ptr;

            prev.ctx.switch_to(&mut next.ctx);
        }
    }
}

impl CoopScheduler {
    /// Initialise the cooperative scheduler. This sets the current PID as the null kernel process,
    /// and creates an empty task table and ready list.
    pub fn new() -> Self {
        CoopScheduler {
            current_pid: AtomicUsize::new(ProcessId::NULL_PROC.inner()),
            task_table: RwLock::new(ProcessList::new()),
            ready_list: RwLock::new(VecDeque::<ProcessId>::new()),
        }
    }
}
