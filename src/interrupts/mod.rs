mod gdt;
#[macro_use]
mod handlers;
use x86::bits64::irq::IdtEntry;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtualAddress;
use memory::MemoryController;

