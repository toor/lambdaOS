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

pub struct Scheduler<'a> {
    //Currently running processes.
    procs: Mutex<BTreeMap<usize, Process>>,
    //Current processes.
    pub current: usize,
    //Number of processes.
    pid_counter: usize,
    skip: usize,
    memory_controller: &'a mut memory::MemoryController,
}

impl <'a> Scheduler<'a> {
    pub fn new(mem_cont: &'a mut memory::MemoryController) -> Self {
        let mut scheduler = Scheduler {
            procs: Mutex::new(BTreeMap::new()),
            current: 0,
            pid_counter: 0,
            skip: 0,
            memory_controller: mem_cont,
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
    
    ///Allocates a stack for the process and creates a new process using a pointer to the top of
    ///the new stack.
    pub fn start_new_process(&mut self, fn_ptr: usize) {
        let proc_stack = self.memory_controller.alloc_stack(256).expect("Could not allocate process stack");
        println!("Top of new process stack: {:x}", proc_stack.top());
        self.create_process(fn_ptr, proc_stack.top());
    }
    
    ///Call Process::new() with the specified parameters. Adds the process to the process table, and returns the PID of this new process.
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
    
    ///Set the specified process as started.
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
    
    ///Update the trap frame of the process to point to the passed address.
    pub fn update_trap_frame(&mut self, trap_frame: usize) {
        self.disable_interrupts();
        {
            let mut procs = self.procs.lock();
            let p = procs.get_mut(&self.current);

            match p {
                None => panic!("Unable to find process {}", self.current),
                //Set the trap frame of the current process to be the address passed as an arg.
                Some(process) => (*process).trap_frame = trap_frame,
            }
        }
    }
    
    ///Lookup the process by its PID. There are a maximum of 5 procs, so if PID == 4, then the PID
    ///of the next process in the table is 0, because we loop round.
    pub fn get_pid(&self) -> Option<usize> {
        let mut pid;
        self.disable_interrupts();
        {
            let procs = self.procs.lock();
            
            if procs.len() == 1 {
                return None;
            }

            pid = match self.current {
                //Max 4 processes for now, we loop round to proc 0 as the next PID.
                4 => Some(0),
                _ => Some(self.current + 1),
            };
        }
        pid
    }

    fn switch(&mut self) {
        //Find a new process that is not currently running.
        let mut process;

        self.disable_interrupts();
        {
            let next = self.get_pid();

            match next {
                Some(p) => {
                    let mut proc_table = self.procs.lock();
                    match proc_table.get_mut(&p) {
                        Some(prc) => {
                            process = prc.clone();
                            if !process.started {
                                (*prc).started = true;
                            }
                        },
                        None => panic!("Unable to find process {}", p),
                    }
                },

                None => return,
            }
        }

        self.current = process.pid;

        if process.started {
            if process.pid == 1 {
                //TODO.
            }
            
            //Jump to the trap frame of the process.
            unsafe {
                asm!("movq $0, %rsp
                      pop    %rax
                      pop    %rbx
                      pop    %rcx
                      pop    %rdx
                      pop    %rsi
                      pop    %rdi
                      pop    %r8
                      pop    %r9
                      pop    %r10
                      pop    %r11
                      pop    %rbp
                      sti
                      iretq" : /* no outputs */ : "r"(process.trap_frame) : );
            }
        } else {
            unsafe {
                asm!("movq $0, %rsp
                      sti
                      jmpq *$1" :: "r"(process.stack), "r"(test_fn as usize) : );
            }
            
            //TODO: Cleanup stack, find a way to exit the process.
        }
    }

    pub fn disable_interrupts(&self) {
        unsafe { asm!("cli") };
    }

    pub fn enable_interrupts(&self) {
        unsafe { asm!("cli") };
    }
    
    ///"Pause" the CPU indefinitely. This is a divergent function.
    pub fn idle(&self) -> ! {
        loop {
            self.halt();
        }
    }

    fn halt(&self) {
        unsafe {
            asm!("hlt");
            asm!("pause");
        }
    }
    

    ///Find a process to switch to.
    pub fn schedule(&mut self) -> usize{
        self.switch();

        0
    }
}

fn test_fn() {}
