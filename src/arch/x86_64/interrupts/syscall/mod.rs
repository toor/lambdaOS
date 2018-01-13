pub mod x86_64;

use x86_64::structures::idt::ExceptionStackFrame;
use syscall;
use arch::interrupts::disable_interrupts_and_then;

#[repr(C, packed)]
pub struct SyscallContext {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
}

pub extern "x86-interrupt" fn syscall_handler(_stack_frame: &mut ExceptionStackFrame) {
    unsafe {
        asm!("cli");
        
        let mut my_sp: usize;
        
        // Assigns the value of the register stack-base pointer to my_sp.
        asm!("" : "={rbp}"(my_sp));

        // Get reference to RAX, the 14th register that the x86-interrupt calling conv pushes to
        // the stack.
        my_sp -= (8 * 13);
    
        // Get reference to stack pointer.
        let sp = my_sp + 0x18;
        
        // Get reference to stack variables.
        let ref ctx: SyscallContext = *(sp as *const SyscallContext);

        let num = ctx.rax;
        let a = ctx.rdi;
        let b = ctx.rsi;
        let c = ctx.rdx;
        let d = ctx.r10;
        let e = ctx.r8;
        let f = ctx.r9;

        // Match against the syscall number.
        let res = syscall::match_syscall(num, a, b, c, d, e, f);
        
        asm!("mov rsp, $0
             mov rax, $1
             pop rbx
             pop rcx
             pop rdx
             pop rsi
             pop rdi
             pop r8
             pop r9
             pop r10
             pop r11
             pop r12
             pop r13
             pop r14
             pop r15
             pop rbp
             sti
             iretq" : /* No outputs */ : "r"(my_sp), "r"(res) : "memory" : "intel", "volatile");
    };
}

#[no_mangle]
pub fn syscall_test() -> u64 {
    unsafe { x86_64::syscall6(16, 32, 128, 64, 256, 512, 1024) } 
}
