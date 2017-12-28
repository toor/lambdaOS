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

unsafe fn disable() {
    asm!("cli");
}

unsafe fn enable() {
    asm!("sti");
}

/// Disable all interrupts and save the PIC masks
pub fn disable_interrupts() -> (u8, u8) {
    use device::pic::PICS;

    unsafe {
        disable();
    }

    let saved_masks: (u8, u8) = {
        let mask_pic0 = PICS.lock().pics[0].data.read();
        let mask_pic1 = PICS.lock().pics[1].data.read();

        (mask_pic0, mask_pic1)
    };

    PICS.lock().pics[0].data.write(0xff);
    PICS.lock().pics[1].data.write(0xff);

    saved_masks
}

/// Enable all interrupts
pub fn enable_interrupts() {
    use device::pic::PICS;

    // Ensure that PIC manipulation is not interrupted
    unsafe {
        disable();
    }

    {
        // Clear all interrupt masks
        PICS.lock().pics[0].data.write(0);
        PICS.lock().pics[1].data.write(0);
    }

    unsafe {
        enable();
    }
}

/// Restore interrupts to previous state
pub fn restore_interrupts(saved_masks: (u8, u8)) {
    use device::pic::PICS;

    // Ensure PIC manipulation is not interrupted
    unsafe {
        disable();
    }

    let (mask_pic0, mask_pic1) = saved_masks;

    PICS.lock().pics[0].data.write(mask_pic0);
    PICS.lock().pics[1].data.write(mask_pic1);

    unsafe {
        enable();
    }
}

// Stolen from Robert Gries.
// This function disables interrupts, allows a function to run without them enabled, and then
// reenables interrupts.
pub fn disable_interrupts_and_then<F, T>(f: F) -> T
    where F: FnOnce() -> T
{
    let saved_masks = disable_interrupts();

    let result: T = f();

    restore_interrupts(saved_masks);

    result
}
