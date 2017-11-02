use spin::Mutex;
use x86::shared::dtables::DescriptorTablePointer;
use x86::shared::dtables;
use x86::bits64::irq::IdtEntry;
use io::ChainedPics;
use x86;

pub static PICS: Mutex<ChainedPics> = Mutex::new(ChainedPics::new(0x20, 0x28));
pub static IDT_INTERFACE: Mutex<Idt> = Mutex::new(Idt::new());

macro_rules! make_idt_entry {
    ($name:ident, $body:expr) => {{
        fn body() {
            $body
        }

        #[naked]
        unsafe extern fn $name() {
            asm!("push rbp
                  push r15
                  push r14
                  push r13
                  push r12
                  push r11
                  push r10
                  push r9
                  push r8
                  push rsi
                  push rdi
                  push rdx
                  push rcx
                  push rbx
                  push rax
                  mov rsi, rsp
                  push rsi
                  
                  cli
                  
                  call $0
                  
                  sti
                  
                  add rsp, 8
                  pop rax
                  pop rbx
                  pop rcx
                  pop rdx
                  pop rdi
                  pop rsi
                  pop r8
                  pop r9
                  pop r10
                  pop r11
                  pop r12
                  pop r13
                  pop r14
                  pop r15
                  pop rbp
                  iretq" :: "s"(body as fn()) :: "volatile" :: "intel");
            ::core::intrinsics::unreachable();
        }

        use x86::shared::paging::VAddr;
        use x86::shared::PrivilegeLevel;

        let handler = VAddr::from_usize($name as usize);

        IdtEntry::new(handler, 0x8, PrivilegeLevel::Ring0, false)
    }};
}


//CPU looks at this table when it wants to know what do do on an interrupt.
static IDT: Mutex<[IdtEntry; 256]> = Mutex::new([IdtEntry::MISSING; 256]);

//Point to this table
pub struct Idt {
    ptr: DescriptorTablePointer<IdtEntry>,
    idt: &'static Mutex<[IdtEntry; 256]>,
}

unsafe impl ::core::marker::Sync for Idt {} 

impl Idt {
    //Create a new pointer to Idt
    pub fn new() -> Idt {
        let idt = Idt {
            ptr: DescriptorTablePointer::new_idtp(&IDT.lock()[..]),
            idt: &IDT,
        };

        unsafe { dtables::lidt(&idt.ptr) };

        idt
    }

    pub fn set_handler(&self, index: usize, entry: IdtEntry) {
        self.idt.lock()[index] = entry;
    }

    pub fn enable_interrupts(&self) {
        unsafe {
            x86::shared::irq::enable();
        }
    }
}


