mod gdt;
#[macro_use]
mod handlers;
use x86::bits64::irq::IdtEntry;
use x86::shared::dtables::lidt;
use x86::shared::dtables;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtualAddress;
use memory::MemoryController;
use self::handlers::IDT;
use io;
use spin::Mutex;

//Our interface to the PICS.
pub static PICS: Mutex<io::ChainedPics> = Mutex::new(unsafe {io::ChainedPics::new(0x20, 0x28) }); 

pub fn initialize() {
    //Load the GDT.
    gdt::Gdt::load();
    
    //Create a pointer to our Idt.
    let idt_ptr = new_idtp(&IDT);
    //Load IDT from this pointer.
    lidt(&idt_ptr);

    //Remap the 8259 PIC.
    PICS.lock().init();
}

pub unsafe fn test_interrupt() {
    asm!("int 0x15" :::: "volatile", "intel");
}
