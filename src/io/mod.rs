use core::marker::PhantomData;

///Command to begin init of the PIC chip.
const CMD_INIT: u8 = 0x11;

///EOI command, that tells the PIC it can begin receiving other interrupts again.
const CMD_END_OF_INTERRUPT: u8 = 0x20;

///The PIC lives in ancient 8086 land.
const MODE_8086: u8 = 0x01;

#[macro_use]
pub mod cpuio;
pub mod serial;
pub mod keyboard;
pub mod vga;
pub mod pic;
//pub mod ata;
use self::cpuio::Port;


