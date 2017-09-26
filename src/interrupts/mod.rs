use core::mem::size_of;
use core::ptr;
use memory;
use io::{keyboard, ChainedPics};
use spin::Mutex;
use x86;
use x86::bits64::irq::IdtEntry;
use x86::shared::descriptor::Flags;

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
    rsi: u64,
    rdi: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rdx: u64,
    rcx: u64,
    rax: u64,
    int_id: u32,
    _pad_1: u32,
    error_code: u32,
    _pad_2: u32,
}

fn cpu_exception_handler(ctx: &InterruptContext) {
    //Print some general info.
    println!("{}, error: 0x{:x}", x86::shared::irq::EXCEPTIONS[ctx.int_id as usize],
             ctx.error_code);

    //Match against error codes we know about and print more info if we have it.
    match ctx.int_id {
        14 => {
            let err = x86::bits64::irq::PageFaultError::from_bits(ctx.error_code);
            println!("{:?}", err);
        }

        _ => {}
    }
    
    //Just put the CPU into a loop until we can do something about this exception.
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn rust_interrupt_handler(ctx: &InterruptContext) {
    match ctx.int_id {
        0x00..0x0F => cpu_exception_handler(ctx),
        0x20 => {/*Timer*/}
        0x21 => {
            if let Some(input) = keyboard::read_char() {
                if input == '\r' {
                    println!("");
                } else {
                    println!("{}", input);
                }
            }
        }
        0x80 => println!("This isn't actually Linux, sorry about that."),
        _ => {
            println!("Unknown interrupt: #{}", ctx.int_id);
            loop {}
        }
    }

    PICS.lock().notify_end_of_interrupt(ctx.int_id as u8);
}

struct Idt {
    table: [IdtEntry; IDT_ENTRY_COUNT],
}

impl Idt {
    pub fn initialize(&mut self) {
        self.add_handlers(); //Add our handler functions.
        self.load();
    }
    
    //Fill in the IDT with our handlers.
    fn add_handlers(&mut self) {
        for (index, &handler) in interrupt_handlers.iter().enumerate() {
            if handler != ptr::null() {
                self.table[index] = IdtEntry::new(gdt64_code_offset, handler);
            }
        }
    }

    unsafe fn load(&self) {
        let pointer = x86::shared::dtables::DescriptorTablePointer {
            base: &self.table[0] as *const IdtEntry,
            limit: (size_of::<IdtEntry>() * IDT_ENTRY_COUNT) as u16,
        };
        x86::shared::dtables::lidt(&pointer);
    }
}

//Global IDT
static IDT: Mutex<Idt> = Mutex::new(Idt {
    table: [missing_handler(); IDT_ENTRY_COUNT]
});

#[allow(dead_code)]
pub unsafe fn test_interrupt() {
    println!("triggering interrupt.");
    int!(0x80);
    println!("Interrupt returned.");

}

pub unsafe fn initialize() {
    //Run PIC init sequence.
    PICS.lock().init();

    //Create a new IDT.
    IDT.lock().initialize();
    
    //Trigger a test interrupt.
    test_interrupt();
    
    //Turn on real interrupts.
    x86::shared::irq::enable();
}

const fn missing_handler() -> IdtEntry {
    IdtEntry {
        base_lo: 0,
        selector: 0,
        reserved0: 0,
        flags: Flags::from_bits(0).unwrap(),
        base_hi: 0,
        reserved1: 0,
    }
}

trait IdtEntryExt {
    fn new(gdt_code_selector: u16, handler: *const u8) -> IdtEntry;
}

impl IdtEntryExt for IdtEntry {
    fn new(gdt_code_selector: u16, handler: *const u8) -> IdtEntry {
        IdtEntry {
            base_lo: ((handler as u64) & 0xFFFF) as u16,
            selector: gdt_code_selector,
            reserved0: 0,
            flags: Flags::from_bits(0b100_01110).unwrap(),
            base_hi: (handler as u64) >> 16,
            reserved1: 0,
        }
    }
}
