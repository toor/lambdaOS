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

        let mut stack: Box<usize> = vec![0; INITIAL_STACK].into_boxed_slice();

        let index: usize = stack.len() - 3;
        let stack_offset: usize = index * mem::size_of::<usize>();

        unsafe {
            let self_idx = stack.as_mut_ptr().offset((stack.len() -1) as isize);
            let self_ptr: *const Scheduler = &*self as *const Scheduler;
            *(self_idx as *mut usize) = self_ptr as usize;

            let ret_ptr = stack.as_mut().offset((stack.len() - 1) as isize);
            *(ret_ptr as *mut usize) = process::process_return as usize;
        }
    }
}
