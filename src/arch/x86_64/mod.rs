pub mod interrupts;
pub mod memory;

use device;

pub unsafe fn kinit(multiboot_info: usize) {
    ::utils::disable_interrupts();
    {
        device::vga::buffer::clear_screen();

        let boot_info = ::multiboot2::load(multiboot_info);

        ::utils::enable_nxe_bit();
        ::utils::enable_write_protect_bit();

        let mut memory_controller = memory::init(&boot_info);

        interrupts::init(&mut memory_controller);

        device::pic::PICS.lock().init();

        device::init();
    }
    ::utils::enable_interrupts();
}
