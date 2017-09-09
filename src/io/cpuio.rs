use core::marker::PhantomData;

mod x86 {
    //Read a single byte from the port.
    pub unsafe fn inb(port: u16) -> u8 {
        let result: u8,
        asm!("inb %dx, %al" : "={al}"(result) : "{dx}"(port) :: "volatile");
        result
    }
    
    //Write a single byte to the port.
    pub unsafe fn outb(port: u16) -> u8 {
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
