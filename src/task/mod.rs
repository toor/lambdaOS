///Credit for most of this code completely goes to Robert Gries (https://github.com/robert-w-gries/rxinu/blob/feature/usermode/src/scheduling/mod.rs). His work on scheduling is really great, check him out.

pub mod context;
pub mod process;
pub mod proc_list;
pub mod coop_sched;

use self::coop_sched as scheduler;

pub use self::process::{Process, ProcessId, State};
pub use self::proc_list::ProcessList;
pub use self::scheduler::Scheduler;
use core::result::Result;
use alloc::string::String;

///Methods a scheduler should impl.
pub trait Scheduling {
    fn create(&self, func: extern "C" fn(), name: String) -> Result<ProcessId, i16>;
    fn get_id(&self) -> ProcessId;
    fn kill(&self, id: ProcessId);
    fn ready(&self, id: ProcessId);
    unsafe fn resched(&self);
}

//Max processes we can handle.
pub const MAX_PROCS: usize = usize::max_value() - 1;

//TODO use max value of u16 as initial stack size.
pub const INITIAL_STACK: usize = 1000;

lazy_static! {
    pub static ref SCHEDULER: Scheduler = Scheduler::new();
}
