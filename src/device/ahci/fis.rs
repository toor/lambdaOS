/// All of these structures are pretty much just adaptations of
/// http://wiki.osdev.org/AHCI#SATA_basic.

use device::mmio::Mmio;

#[repr(u8)]
pub enum FisType {
    RegH2D = 0x27,
    RegD2H = 0x34,
    DmaAct = 0x39,
    DmaSetup = 0x41,
    Data = 0x46,
    Bist = 0x58,
    PioSetup = 0x5F,
    DevBits = 0xA1,
}
