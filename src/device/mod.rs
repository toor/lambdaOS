#[macro_use]
/// Primary interface to I/O ports - special memory addresses on a different bus that we can use to
/// access I/O devices.
pub mod io;
/// Simple driver for the PS/2 keyboard.
pub mod keyboard;
/// New interface to PS/2.
pub mod ps2_8042;
/// Advanced split interface to VGA buffer split between text management and actual buffer
/// addressing.
pub mod vga;
/// The 8259 PIC - Programmable Interrupt Controller. Contains data structures that we can use to
///control the behaviour of this device.
pub mod pic;
/// PIT controller.
pub mod pit;
/// AHCI driver.
pub mod ahci;
/// PCI functionality
pub mod pci;

pub use self::io::cpuio::{Port, UnsafePort};
pub use self::io::mmio;

/// Perform hardware init.
pub unsafe fn init() {
    vga::init();
    pic::PICS.lock().init();
    pit::init();
    ps2_8042::PS2.lock().init();
    pci::init();
}
