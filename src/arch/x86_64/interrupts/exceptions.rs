//! Handlers for internal CPU exceptions. Currently, when an exception occurs, we just print some
//! debug information and then spin the CPU. TODO: Figure out which exceptions are safe to return
//! from.

use x86_64::structures::idt::{ExceptionStackFrame, PageFaultErrorCode};
use super::disable_interrupts_and_then;

/// Handler for the #DE Exception. This exception occurs when divinding any number by zero using
/// either the DIV or IDIV instructions.
pub extern "x86-interrupt" fn divide_by_zero_handler(stack_frame: &mut ExceptionStackFrame) {
    disable_interrupts_and_then(|| {
        println!("\nEXCEPTION: DIVIDE BY ZERO\n{:#?}", stack_frame);
        loop {}
    });
}

/// The Debug exception occurs under the following conditions. It is either a fault or a trap.
/// - Instruction fetch breakpoint (Fault).
/// - General detect condition (Fault).
/// - Data r/w breakpoint (Trap).
/// - I/O r/w breakpoint (Trap).
/// - Single-step (Trap).
/// - Task switch (Trap).
pub extern "x86-interrupt" fn debug_handler(stack_frame: &mut ExceptionStackFrame) {
    disable_interrupts_and_then(|| {
        println!("\nEXCEPTION: DEBUG\n{:#?}", stack_frame);
        loop {}
    });
}

/// A non-maskable interrupt is a hardware-driven interrupt much like those sent by the PIC, except
/// an NMI either goes directly to the CPU or via another controller. An NMI occurs for hardware
/// errors, which are something we can do nothing about. TODO: Investigate how we might discover
/// which piece of hardware is faulty.
pub extern "x86-interrupt" fn nmi_handler(stack_frame: &mut ExceptionStackFrame) {
    disable_interrupts_and_then(|| {
        println!("\nEXCEPTION: NON-MASKABLE INTERRUPT\n{:#?}", stack_frame);
        loop {}
    });
}

/// Hardware breakpoint exception. This can return without issues.
pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    println!(
        "\nEXCEPTION: BREAKPOINT at {:#x}\n{:#?}",
        stack_frame.instruction_pointer, stack_frame
    );
}

/// An overflow exception occurs in two situations - where an INTO instruction is executed and the
/// OVERFLOW bit in RFLAGS is set to 1, or when the result of `DIV/IDIV` instruction is greater
/// than the maximum value of a 64-bit integer.
pub extern "x86-interrupt" fn overflow_handler(stack_frame: &mut ExceptionStackFrame) {
    disable_interrupts_and_then(|| {
        println!("\nEXCEPTION: OVERFLOW\n{:#?}", stack_frame);
        loop {}
    });
}

/// A Bound Range Exceeded exception occurs when the `BOUND` instruction is executed and the index is
/// out of bounds. The `BOUND` instruction takes an index into an array, and compares it with the
/// upper and lower bounds of the array. If the index is out of bounds, this exception is thrown.
pub extern "x86-interrupt" fn bound_range_handler(stack_frame: &mut ExceptionStackFrame) {
    disable_interrupts_and_then(|| {
        println!("\nEXCEPTION: BOUND RANGE EXCEEDED\n{:#?}", stack_frame);
        loop {}
    });
}

/// If the processor tries to execute an instruction with an invalid or undefined exception (or if
/// the instruction exceeds 15 bytes), an `INVALID OPCODE` exception is thrown.
pub extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: &mut ExceptionStackFrame) {
    disable_interrupts_and_then(|| {
        println!(
            "\nEXCEPTION: INVALID OPCODE at {:#x}\n{:#?}",
            stack_frame.instruction_pointer, stack_frame
        );
        loop {}
    });
}

/// This exception occurs when the processor tries to execute an FPU-related instruction but there
/// is no x87 present. This is a very rare occurence, as only very old hardware will not have an
/// FPU.
pub extern "x86-interrupt" fn device_not_available_handler(stack_frame: &mut ExceptionStackFrame) {
    disable_interrupts_and_then(|| {
        println!("\nEXCEPTION: FPU NOT AVAILABLE\n{:#?}", stack_frame);
        loop {}
    });
}

/// A Double Fault occurs when a) an exception is unhandled, b) when an exception occurs whilst the
/// CPU is in the process of calling the exception handler for the first exception. This is an
/// Abort, meaning it is not possible to recover from a Double Fault.
pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut ExceptionStackFrame,
    _error_code: u64,
) {
    disable_interrupts_and_then(|| {
        println!("\nEXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
        loop {}
    });
}

/// The Invalid TSS exception occurs when an invalid segment selector is referenced during
/// control transfer through a gate descriptor.
pub extern "x86-interrupt" fn invalid_tss_handler(stack_frame: &mut ExceptionStackFrame, error_code: u64) {
    disable_interrupts_and_then(|| {
        println!("\nEXCEPTION: INVALID TSS with code: {:?}\n{:#?}", error_code, stack_frame);
        loop {}
    });
}

/// A Segment Not Present exception occurs when an attempt is made to load a segment which has its
/// present bit set to 0.
pub extern "x86-interrupt" fn seg_not_present_handler(stack_frame: &mut ExceptionStackFrame,
                                                      error_code: u64)
{
    disable_interrupts_and_then(|| {
        println!("\nEXCEPTION: SEGMENT NOT PRESENT\nerror code: \
                 {:?}\n{:#?}", error_code, stack_frame);

        loop {}
    });
}

/// A Stack Segment Fault exception occurs when:
/// - Loading a stack segment referencing a non-present segment descriptor.
/// - PUSH/POP using ESP/EBP where the referenced address is non-canonical.
/// - The stack limit check fails.
pub extern "x86-interrupt" fn stack_seg_fault_handler(stack_frame: &mut ExceptionStackFrame,
                                                      error_code: u64)
{
    disable_interrupts_and_then(|| {
        println!("\nEXCEPTION: STACK SEGMENT FAULT\nerror code: \
                 {:?}\n{:#?}", error_code, stack_frame);

        loop {}
    });
}

/// A General Protection Fault can occur for several different reasons.
/// - Segment error (privilege, type, limit, read/write rights).
/// - Executing a privileged instruction (IRET, INT, OUT, etc.), while CPL != 0.
/// - Writing 1 into a reserved register field.
/// - Referencing the null segment descriptor.
/// - Trying to access an unimplemented register (i.e in Protected Mode: `mov cr6, eax` is
/// illegal).
pub extern "x86-interrupt" fn gpf_handler(stack_frame: &mut ExceptionStackFrame, _error_code: u64) {
    disable_interrupts_and_then(|| {
        println!("\nEXCEPTION: GPF\n{:#?}", stack_frame);
        loop {}
    });
}

/// A Page Fault occurs when:
/// - a page directory or table entry is not present in physical memory.
/// - An attempt to load the TLB with a translation for a non-executable page occurs.
/// - A protection check on the page (r/w, priveleges) failed.
/// - A reserved bit in the page directory or table entries is set to 1.
/// The address that the CPU tried to access is saved in register `cr2`.
pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: &mut ExceptionStackFrame,
    error_code: PageFaultErrorCode,
) {
    disable_interrupts_and_then(|| {
        use x86_64::registers::control_regs;
        println!(
            "\nEXCEPTION: PAGE FAULT while accessing {:#x}\nerror code: \
             {:?}\n{:#?}",
            control_regs::cr2(),
            error_code,
            stack_frame
        );
        loop {}
    });
}

/// An x87-floating point exception occurs when any waiting floating point instruction (e.g, FWAIT
/// or WAIT.), and the following conditions are true:
/// - CR0.NE = 1,
/// - an unmasked x87 floating point exception is pending.
pub extern "x86-interrupt" fn x87_fp_exception_handler(stack_frame: &mut ExceptionStackFrame) {
    disable_interrupts_and_then(|| {
        println!("\nX87 FLOATING POINT EXCEPTION\n{:#?}", stack_frame);
        loop {}
    });
}

/// The Alignment Check exception occurs when alignment checking is enabled and a instruction
/// attempts to reference an unaligned memory address. Alignment checking is only performed if
/// `CPL = 3`.
pub extern "x86-interrupt" fn alignment_check_handler(stack_frame: &mut ExceptionStackFrame, _error_code: u64) {
    disable_interrupts_and_then(|| {
        println!("\nEXCEPTION: ALIGNMENT CHECK\n{:#?}", stack_frame);
        loop {}
    });
}

/// The Machine Check exception is an exception that occurs when the CPU detects that it has
/// internal errors - i.e, bad memory, bad cache, faulty timings etc. The error information is
/// placed in the model-specific registers.
pub extern "x86-interrupt" fn machine_check_handler(stack_frame: &mut ExceptionStackFrame) {
    disable_interrupts_and_then(|| {
        // TODO: use the MSRs to get error information about the MC.
        println!("\nEXCEPTION: MACHINE CHECK\n{:#?}", stack_frame);
        loop {}
    });
}

/// If the `CR4.OSXMMEXCEPT` bit is set to 1 in `cr4`, then an unmasked 128-bit media instruction
/// will cause this exception. Otherwise, an `Invalid Opcode` exception occurs.
pub extern "x86-interrupt" fn simd_fp_exception_handler(stack_frame: &mut ExceptionStackFrame) {
    disable_interrupts_and_then(|| {
        println!("\nEXCEPTION: SIMD FLOATING POINT EXCEPTION\n{:#?}", stack_frame);
        loop {}
    });
}
