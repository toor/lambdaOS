///Credit for most of this code completely goes to Robert Gries (https://github.com/robert-w-gries/rxinu/blob/feature/usermode/src/scheduling/mod.rs). His work on scheduling is really great, check him out.

pub mod context;
pub mod process;
pub mod proc_list;
pub mod coop_sched;

use self::coop_sched as scheduler;

pub use self::process::{Process, ProcessId, State};
pub use self::proc_list::ProcessList;
use core::result::Result;

///Methods a scheduler should impl.
pub trait Scheduling {
    fn create(&self, func: extern "C" fn()) -> Result<ProcessId, i16>;
    fn get_id(&self) -> ProcessId;
    fn kill(&self, id: ProcessId);
    fn ready(&self, id: ProcessId);
    unsafe fn resched(&self);
}

//Max processes we can handle.
pub const MAX_PROCS: usize = usize::max_value() - 1;

pub const INITIAL_STACK: usize = 65536;
