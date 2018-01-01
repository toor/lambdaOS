use alloc::VecDeque;
use alloc::vec::Vec;
use alloc::String;
use core::mem;
use core::ops::DerefMut;
use core::sync::atomic::{AtomicUsize, Ordering};
use task::{Scheduling, Process, ProcessId, ProcessList, State, INITIAL_STACK};
use task::process;
use spin::RwLock;

pub type Scheduler = CoopScheduler;

pub struct CoopScheduler {
    current_pid: AtomicUsize,
    task_t: RwLock<ProcessList>,
    ready_list: RwLock<VecDeque<ProcessId>>,
}

impl Scheduling for CoopScheduler {
    fn create(&self, func: extern "C" fn(), name: String) -> Result<ProcessId, i16> {
        use arch::memory::paging;

        let mut stack: Vec<usize> = vec![0; INITIAL_STACK];

        let proc_top: usize = stack.len() - 3;

        let proc_sp = stack.as_ptr() as usize + (proc_top * mem::size_of::<usize>());
        
        use alloc::boxed::Box;
        let self_ptr: Box<&Scheduling> = Box::new(self);

        let stack_vals: Vec<usize> = vec![
            func as usize,
            process::process_return as usize,
            Box::into_raw(self_ptr) as usize,
        ];

        for (i, val) in stack_vals.iter().enumerate() {
            stack[proc_top + i] = *val;
        }

        let mut proc_t_lock = self.task_t.write();

        let proc_lock = proc_t_lock.add()?;
        {
            let mut process = proc_lock.write();

            process.stack = Some(stack);
            process.name = name;

            process.ctx.set_page_table(unsafe { paging::ActivePageTable::new().address() });

            process.ctx.set_stack(proc_sp);

            Ok(process.pid)
        }
    }

    fn get_id(&self) -> ProcessId {
        ProcessId(self.current_pid.load(Ordering::SeqCst))
    }

    fn kill(&self, id: ProcessId) {
        //We should free the stack here.
        {
            let task_table_lock = self.task_t.read();
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

    fn ready(&self, id: ProcessId) {
        self.ready_list.write().push_back(id);
    }

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
            let task_table_lock = self.task_t.read();
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

                    self.current_pid
                        .store(next.pid.inner(), Ordering::SeqCst);

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
    pub fn new() -> Self {
        CoopScheduler {
            current_pid: AtomicUsize::new(ProcessId::NULL_PROC.inner()),
            task_t: RwLock::new(ProcessList::new()),
            ready_list: RwLock::new(VecDeque::<ProcessId>::new()),
        }
    }
}
