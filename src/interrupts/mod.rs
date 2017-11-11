use memory::MemoryController;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::idt::{ExceptionStackFrame, Idt, PageFaultErrorCode};
use spin::{Once, Mutex};
use io::ChainedPics;

mod gdt;
mod exceptions;
mod irq;

const DOUBLE_FAULT_IST_INDEX: usize = 0;

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();

        //Set exception handlers.
        idt.divide_by_zero.set_handler_fn(exceptions::divide_by_zero_handler);
        idt.breakpoint.set_handler_fn(exceptions::breakpoint_handler);
        idt.invalid_opcode.set_handler_fn(exceptions::invalid_opcode_handler);
        idt.page_fault.set_handler_fn(exceptions::page_fault_handler);
        idt.general_protection_fault.set_handler_fn(exceptions::gpf_handler);

        unsafe {
            idt.double_fault.set_handler_fn(exceptions::double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX as u16);
        }

        idt
    };
}

static TSS: Once<TaskStateSegment> = Once::new();
static GDT: Once<gdt::Gdt> = Once::new();

pub fn init(memory_controller: &mut MemoryController) {
    use x86_64::structures::gdt::SegmentSelector;
    use x86_64::instructions::segmentation::set_cs;
    use x86_64::instructions::tables::load_tss;
    use x86_64::VirtualAddress;

    let dfault_stack = memory_controller
        .alloc_stack(1)
        .expect("Failed to allocate double fault stack");

    let tss = TSS.call_once(|| {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX] =
            VirtualAddress(dfault_stack.top());
        tss
    });

    let mut code_selector = SegmentSelector(0);
    let mut tss_selector = SegmentSelector(0);

    let gdt = GDT.call_once(|| {
        let mut gdt = gdt::Gdt::new();

        code_selector = gdt.add_entry(gdt::Descriptor::kernel_code_segment());
        tss_selector = gdt.add_entry(gdt::Descriptor::tss_segment(&tss));
        gdt
    });

    //Load gdt into memory.
    gdt.load();

    unsafe {
        //Reload cs - the code segment register.
        set_cs(code_selector);

        load_tss(tss_selector);
    }

    IDT.load();
}

//Our interface to the 8259 PIC.
pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(0x20, 0x28) });
