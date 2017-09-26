use core::marker::PhantomData;

mod x86_io {
    //Read a single byte from the port.
    pub unsafe fn inb(port: u16) -> u8 {
        let result: u8;
        asm!("inb %dx, %al" : "={al}"(result) : "{dx}"(port) :: "volatile");
        result
    }
    
    //Write a single byte to the port.
    pub unsafe fn outb(value: u8, port: u16) {
        asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(value) :: "volatile");
    }
    
    //Read a u16 sized value from the port
    pub unsafe fn inw(port: u16) -> u16 {
        let result: u16;
        asm!("inw %dx, %ax" : "={ax}"(result) : "{dx}"(port) :: "volatile");
        result
    }

    // Write a u16-sized value to port.
    pub unsafe fn outw(value: u16, port: u16) {
        asm!("outw %ax, %dx" :: "{dx}"(port), "{ax}"(value) :: "volatile");
    }

    // Read a u32-sized value from port.
    pub unsafe fn inl(port: u16) -> u32 {
        let result: u32;
        asm!("inl %dx, %eax" : "={eax}"(result) : "{dx}"(port) :: "volatile");
        result
    }

    // Write a u32-sized value to port.
    pub unsafe fn outl(value: u32, port: u16) {
        asm!("outl %eax, %dx" :: "{dx}"(port), "{eax}"(value) :: "volatile");
    }
}

use self::x86_io::{inb, outb, inw, outw, inl, outl};

pub trait InOut {
    unsafe fn port_in(port: u16) -> Self;
    unsafe fn port_out(port: u16, value: Self);
}

impl InOut for u8 {
    unsafe fn port_in(port: u16) -> u8 { inb(port) }
    unsafe fn port_out(port: u16, value: u8) { outb(value, port); }
}

impl InOut for u16 {
    unsafe fn port_in(port: u16) -> u16 { inw(port) }
    unsafe fn port_out(port: u16, value: u16) { outw(value, port); }
}

impl InOut for u32 {
    unsafe fn port_in(port: u16) -> u32 { inl(port) }
    unsafe fn port_out(port: u16, value: u32) { outl(value, port); }
}

#[derive(Debug)]
pub struct Port<T: InOut> {
    // Port address.
    port: u16,

    // Zero-byte placeholder.  This is only here so that we can have a
    // type parameter `T` without a compiler error.
    phantom: PhantomData<T>,
}

impl<T: InOut> Port<T> {
    pub const unsafe fn new(port: u16) -> Port<T> {
        Port { port: port, phantom: PhantomData }
    }

    pub fn read(&mut self) -> T {
        unsafe { T::port_in(self.port) }
    }

    pub fn write(&mut self, value: T) {
        unsafe { T::port_out(self.port, value); }
    }
}

#[derive(Debug)]
pub struct UnsafePort<T: InOut> {
    port: u16,
    phantom: PhantomData<T>,
}

impl<T: InOut> UnsafePort<T> {
    // Create a new I/O port.
    pub const unsafe fn new(port: u16) -> UnsafePort<T> {
        UnsafePort { port: port, phantom: PhantomData }
    }

    // Read data from the port.
    pub unsafe fn read(&mut self) -> T {
        T::port_in(self.port)
    }

    // Write data to the port.
    pub unsafe fn write(&mut self, value: T) {
        T::port_out(self.port, value);
    }
}
