use core::marker::PhantomData;

#[macro_use]
///Primary interface to I/O ports - special memory addresses on a different bus that we can use to
///access I/O devices.
pub mod cpuio;
///Serial driver.
pub mod serial;
///Simple driver for the PS/2 keyboard.
pub mod keyboard;
///Advanced split interface to VGA buffer split between text management and actual buffer
///addressing. 
pub mod vga;
///The 8259 PIC - Programmable Interrupt Controller. Contains data structures that we can use to
///control the behaviour of this device.
pub mod pic;
