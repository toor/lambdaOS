use x86_64::structures::idt::{ExceptionStackFrame, PageFaultErrorCode};

pub extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut ExceptionStackFrame)
{
    println!("Exception: Breakpoint\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn divide_by_zero_handler(
    stack_frame: &mut ExceptionStackFrame)
{
    println!("Exception: Divide by zero\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut ExceptionStackFrame, _error_code: u64)
{
    println!("Exception: double fault\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn invalid_opcode_handler(
    stack_frame: &mut ExceptionStackFrame)
{
    println!("Invalid opcode at {:#?}\n{:#?}",
                stack_frame.instruction_pointer,
                stack_frame);
}

pub extern "x86-interrupt" fn page_fault_handler(stack_frame: &mut ExceptionStackFrame, error_code: PageFaultErrorCode)
{
    use x86_64::registers::control_regs;

    println!("Exception: Page fault while accessing {:#?}\nerror code:{:#?}\n{:#?}", control_regs::cr2(), error_code, stack_frame);

    loop {}
}
