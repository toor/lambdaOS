#[macro_use]
pub mod io;
pub mod keyboard;
pub mod ps2_8042;
pub mod vga;
pub mod pic;
pub mod pit;
pub mod ahci;
pub mod pci;
pub mod apic;
pub mod serial;

pub use self::io::cpuio::{Port, UnsafePort};
pub use self::io::mmio;

use raw_cpuid::CpuId;

/// Perform hardware init.
pub unsafe fn init() {
    vga::init();
    pit::init();
    ps2_8042::PS2.lock().init();
    pci::init();
}
