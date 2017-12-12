use alloc::VecDeque;
use alloc::boxed::Box;
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
    fn create(&self, func: extern "C" fn()) -> Result<ProcessId, i16> {
        use memory::paging;

        let mut stack: Box<[usize]> = vec![0; INITIAL_STACK].into_boxed_slice();

        let index: usize = stack.len() - 3;
        let stack_offset: usize = index * mem::size_of::<usize>();

        unsafe {
            let self_idx = stack.as_mut_ptr().offset((stack.len() -1) as isize);
            let self_ptr: *const Scheduler = &*self as *const Scheduler;
            *(self_idx as *mut usize) = self_ptr as usize;

            let ret_ptr = stack.as_mut_ptr().offset((stack.len() - 1) as isize);
            *(ret_ptr as *mut usize) = process::process_return as usize;

            let func_ptr = stack.as_mut_ptr().offset((stack.len() - 1) as isize);
            *(func_ptr as *mut usize) = func as usize;
        }

        let mut task_table_lock = self.task_t.write();

        let process_lock = task_table_lock.add()?;
        {
            let mut process = process_lock.write();

            process
                .ctx
                .set_page_table(unsafe {paging::ActivePageTable::new().address() });

            process
                .ctx
                .set_stack((stack.as_ptr() as usize) + stack_offset);

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
            let proc_lock = task_table_lock
                .get(id)
                .expect("Cannot kill a non-existent process");

            let mut killed_process = proc_lock.write();
            
            //Just mark the state as free.
            killed_process.set_state(State::Free);
        }

        unsafe {
            self.resched();
        }
    }

    fn ready(&self, id: ProcessId) {
        self.ready_list.write().push_back(id);
    }

    unsafe fn resched(&self) {
        let mut old_ptr = 0 as *mut Process;
        let mut next_ptr = 0 as *mut Process;

        // Separate the locks from the context switch through scoping
        {
            let task_table_lock = self.task_t.read();
            let mut ready_list_lock = self.ready_list.write();

            let curr_id: ProcessId = self.get_id();
            let mut old = task_table_lock
                .get(curr_id)
                .expect("Could not find old process")
                .write();

            if old.state == State::Current {
                old.set_state(State::Ready);
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
                    old_ptr = old.deref_mut() as *mut Process;
                    next_ptr = next.deref_mut() as *mut Process;
                }
            }
        }

        if next_ptr as usize != 0 {
            assert!(
                old_ptr as usize != 0,
                "Pointer to new proc has not been set!"
            );

            (&mut *old_ptr)
                .ctx
                .switch_to(&mut (&mut *next_ptr).ctx);
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
