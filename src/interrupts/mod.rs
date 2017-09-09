use core::mem::size_of;
use core::ptr;
use memory;
use constants::keyboard::KEYBOARD_INTERRUPT;
use io::{ChainedPics};
use spin::Mutex;
use x86;
use x86::shared::irq::IdtEntry;

//Wrap the PICS static in a mutex to avoid data races.
pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(0x20, 0x28) });
const IDT_ENTRY_COUNT: usize = 256;

extern {
    static gdt64_code_offset: u16;

    fn report_interrupt();
    
    static interrupt_handlers: [*const u8; IDT_ENTRY_COUNT];
}

#[repr(C, packed)]
pub struct InterruptContext {
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
