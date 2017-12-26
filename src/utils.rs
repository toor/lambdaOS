pub fn enable_nxe_bit() {
    use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

pub fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};

    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}

pub unsafe fn disable_interrupts() {
    asm!("cli");
}

pub unsafe fn enable_interrupts() {
    asm!("sti");
}

// Stolen from Robert Gries.
// This function disables interrupts, allows a function to run without them enabled, and then
// reenables interrupts.
pub fn disable_interrupts_and_then<F, T>(f: F) -> T
    where F: FnOnce() -> T
{
    unsafe {
        disable_interrupts();

        let result: T = f();

        enable_interrupts();

        result
    }
}
