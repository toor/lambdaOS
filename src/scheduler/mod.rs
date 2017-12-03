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

pub struct Scheduler {
    //Currently running processes.
    procs: Mutex<BTreeMap<usize, Process>>,
    //Current processes.
    pub current: usize,
    //Number of processes.
    pid_counter: usize,
    skip: usize,
}

impl Scheduler {
    pub fn new() -> Self {
        let mut scheduler = Scheduler {
            procs: Mutex::new(BTreeMap::new()),
            current: 0,
            pid_counter: 0,
            skip: 0,
        };

        scheduler.init();
        scheduler
    }

    fn init(&mut self) {
        //Kernel thread - 0th proc, no stack to jump to.
        let pid = self.create_process(0, 0);
        self.current = 0;
        self.set_started(pid);

        {
            let procs = self.procs.lock();
            let process= procs.get(&pid);
            println!("Initialised proc 0 to {:?}", process);
        }
    }

    pub fn start_new_process(&mut self, fn_ptr: usize) {
        //TODO.
    }

    pub fn create_process(&mut self, start_fn: usize, stack_pointer: usize) -> usize {
        let mut pid;
        self.disable_interrupts();
        {
            //Initialise process 0 for the kernel thread.
            let p = Process::new(self.pid_counter, start_fn, stack_pointer);
            //Insert this procees into the process list.
            self.procs.lock().insert(self.pid_counter, p);
            println!("Inserted proc {}, there are {} procs", self.pid_counter, self.procs.lock().len());
            pid = self.pid_counter;
            self.pid_counter += 1;
        }
        
        //Return the PID of this new proc.
        pid
    }

    pub fn set_started(&mut self, pid: usize) {
       self.disable_interrupts();
       {
            //Lookup the process in the proc table.
            let mut procs = self.procs.lock();
            let p = procs.get_mut(&pid);
            match p {
                None => panic!("Unable to get process {}", pid),
                Some(process) => (*process).started = true,
            }
       }
    }

    pub fn update_trap_frame(&mut self, trap_frame: usize) {
        //TODO: Jump to the specified trap frame.
    }

    pub fn switch(&mut self) {
        //TODO: Add context switching.
    }

    pub fn disable_interrupts(&self) {
        unsafe { asm!("cli") };
    }

    pub fn enable_interrupts(&self) {
        unsafe { asm!("cli") };
    }
}
