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
    
    if CpuId::new().get_feature_info().unwrap().has_apic() {
        pic::PICS.lock().disable_8259_pic();
        apic::init();
    } else {
        pic::PICS.lock().init();
    }
    
    pit::init();
    ps2_8042::PS2.lock().init();
    pci::init();
}
