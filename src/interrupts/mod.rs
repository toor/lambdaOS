mod gdt;
#[macro_use]
mod handlers;
use x86::bits64::irq::IdtEntry;
use x86::shared::dtables::lidt;
use x86::shared::dtables::DescriptorTablePointer::new_idtp;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtualAddress;
use memory::MemoryController;
use handlers::IDT;

pub fn initialize() {
    //Load the GDT.
    gdt::Gdt::load();
    
    //Create a pointer to our Idt.
    let idt_ptr = new_idtp(&IDT);
    //Load IDT from this pointer.
    lidt(&idt_ptr);
}
