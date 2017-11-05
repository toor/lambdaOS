use memory::MemoryController;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::idt::{ExceptionStackFrame, Idt, PageFaultErrorCode};
use spin::Once;

mod gdt;
mod exceptions;

const DOUBLE_FAULT_IST_INDEX: usize = 0;

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();

        //Set exception handlers.
        idt.breakpoint.set_handler_fn(exceptions::breakpoint_handler);
        idt.divide_by_zero.set_handler_fn(exceptions::divide_by_zero_handler);
        idt.invalid_opcode.set_handler_fn(exceptions::invalid_opcode_handler);
        idt.page_fault.set_handler_fn(exceptions::page_fault_handler);

        unsafe {
            idt.double_fault.set_handler_fn(exceptions::double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX as u16);
        }

        idt
    };
}
