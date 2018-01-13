#[derive(Clone, Debug)]
pub struct Context {
    pub cr3: usize,
    rbp: usize,
    rflags: usize,
    pub rsp: usize,
    rbx: usize,
    r12: usize,
    r13: usize,
    r14: usize,
    r15: usize,
}

impl Context {
    pub fn new() -> Self {
        Context {
            //Init all fields as 0.
            cr3: 0,
            rbp: 0,
            rflags: 0,
            rsp: 0,
            rbx: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
        }
    }

    ///Switch to the new context.
    #[naked]
    #[inline(never)]
    pub unsafe extern "C" fn switch_to(&mut self, next: &mut Context) {
        asm!("pushfq ; pop $0" : "=r"(self.rflags) : : "memory" : "intel", "volatile");
        asm!("push $0 ; popfq" : "=r"(self.rflags) : : "memory" : "intel", "volatile");

        asm!("mov $0, cr3" : "=r"(self.cr3) : : "memory" : "intel", "volatile");
        asm!("mov $0, rbx" : "=r"(self.rbx) : : "memory" : "intel", "volatile");
        asm!("mov $0, r12" : "=r"(self.r12) : : "memory" : "intel", "volatile");
        asm!("mov $0, r13" : "=r"(self.r13) : : "memory" : "intel", "volatile");
        asm!("mov $0, r14" : "=r"(self.r14) : : "memory" : "intel", "volatile");
        asm!("mov $0, r15" : "=r"(self.r15) : : "memory" : "intel", "volatile");
        asm!("mov $0, rsp" : "=r"(self.rsp) : : "memory" : "intel", "volatile");
        asm!("mov $0, rbp" : "=r"(self.rbp) : : "memory" : "intel", "volatile");

        if next.cr3 != self.cr3 {
            asm!("mov cr3, $0" : : "r"(next.cr3) : "memory" : "intel", "volatile");
        }

        asm!("mov rbx, $0" : : "r"(next.rbx) : "memory" : "intel", "volatile");
        asm!("mov r12, $0" : : "r"(next.r12) : "memory" : "intel", "volatile");
        asm!("mov r13, $0" : : "r"(next.r13) : "memory" : "intel", "volatile");
        asm!("mov r14, $0" : : "r"(next.r14) : "memory" : "intel", "volatile");
        asm!("mov r15, $0" : : "r"(next.r15) : "memory" : "intel", "volatile");
        asm!("mov rsp, $0" : : "r"(next.rsp) : "memory" : "intel", "volatile");
        asm!("mov rbp, $0" : : "r"(next.rbp) : "memory" : "intel", "volatile");
    }

    ///Set page table of this context.
    pub fn set_page_table(&mut self, address: usize) {
        self.cr3 = address;
    }

    ///Set stack pointer.
    pub fn set_stack(&mut self, address: usize) {
        self.rsp = address;
    }
}
