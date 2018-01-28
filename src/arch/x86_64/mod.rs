pub mod interrupts;
pub mod memory;

use device;
use self::memory::MemoryController;

pub static mut MEMORY_CONTROLLER: Option<MemoryController> = None;

pub unsafe fn memory_controller() -> &'static mut MemoryController {
    match MEMORY_CONTROLLER {
        Some(ref mut m) => m,
        None => panic!("Memory controller called before init."),
    }
}

pub unsafe fn kinit(multiboot_info: usize) {
    interrupts::disable_interrupts();
    
    {
        device::vga::buffer::clear_screen();

        let boot_info = ::multiboot2::load(multiboot_info);

        enable_nxe_bit();
        enable_write_protect_bit();

        let mut memory_controller = memory::init(&boot_info);

        interrupts::init(&mut memory_controller);

        MEMORY_CONTROLLER = Some(memory_controller);

        device::pic::PICS.lock().init();

        device::init();
    }

    interrupts::enable_interrupts();
}

pub fn enable_nxe_bit() {
    use x86_64::registers::msr::{rdmsr, wrmsr, IA32_EFER};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

pub fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{Cr0, cr0, cr0_write};

    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}
