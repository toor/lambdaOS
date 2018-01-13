use alloc::btree_map::{self, BTreeMap};
use alloc::vec::Vec;
use alloc::arc::Arc;
use core::result::Result;
use spin::RwLock;
use task::{Process, ProcessId, State};

pub struct ProcessList {
    //Each entry is a PID attached to a locked process.
    procs: BTreeMap<ProcessId, Arc<RwLock<Process>>>,
    next: usize,
}

impl ProcessList {
    pub fn new() -> Self {
        let mut list: BTreeMap<ProcessId, Arc<RwLock<Process>>> = BTreeMap::new();

        // The inital kernel thread, with pid 0.
        let mut null_proc: Process = Process::new(ProcessId::NULL_PROC);
        null_proc.state = State::Current;
        null_proc.stack = Some(Vec::new());

        //Insert this process into the list.
        list.insert(ProcessId::NULL_PROC, Arc::new(RwLock::new(null_proc)));

        ProcessList {
            procs: list,
            next: 1,
        }
    }

    ///Retrieve the given process from the task table.
    pub fn get(&self, id: ProcessId) -> Option<&Arc<RwLock<Process>>> {
        self.procs.get(&id)
    }

    ///Transform process collection into iterator.
    pub fn iter(&self) -> btree_map::Iter<ProcessId, Arc<RwLock<Process>>> {
        self.procs.iter()
    }

    ///Add a process to the task table.
    pub fn add(&mut self) -> Result<&Arc<RwLock<Process>>, i16> {
        //Reset search if we're at the end of the table.
        if self.next >= super::MAX_PROCS {
            self.next = 1;
        }

        while self.procs.contains_key(&ProcessId(self.next)) {
            self.next += 1;
        }

        if self.next >= super::MAX_PROCS {
            Err(-1)
        } else {
            let id: ProcessId = ProcessId(self.next);
            self.next += 1;

            assert!(
                self.procs
                    .insert(id, Arc::new(RwLock::new(Process::new(id))))
                    .is_none(),
                "Process already exists"
            );

            Ok(self.procs.get(&id).expect("Failed to add new process."))
        }
    }

    ///Remove process from task table.
    pub fn remove(&mut self, id: ProcessId) -> Option<Arc<RwLock<Process>>> {
        self.procs.remove(&id)
    }
}
