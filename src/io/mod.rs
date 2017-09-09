use core::marker::PhantomData;
use x86::shared::io::{inl, outl, outw, inw, outb, inb};
use event;

//Begin init of the PIC chip
const CMD_INIT: u8 = 0x11;

//Interrupt acknowledgement
const CMD_END_OF_INTERRUPT: u8 = 0x20;

//PIC mode
const MODE_8086: u8 = 0x01;

#[macro_use]
pub mod keyboard;
pub mod drivers;

struct Pic {
    offset: u8,
    command: Port<u8>, //Commands are sent on one address and data on the other
    data: Port<u8>,
}

impl Pic {
    fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        self.offset <= interrupt_id && interrupt_id < self.offset + 8
    } //Does this PIC handle interrupts?

    unsafe fn end_of_interrupt(&mut self) {
        self.command.write(CMD_END_OF_INTERRUPT);
    } //Write the magic constant to the port to tell it the interrupt is over
}

pub struct ChainedPics {
    pics: [Pic; 2],
}

impl ChainedPics {
    pub const unsafe fn new(offset1: u8, offset2: u8) -> ChainedPics {
        ChainedPics {
            pics: [Pic {
                offset: offset1,
                command: Port::new(0x20),
                data: Port::new(0x21),
            },
            Pic {
                offset: offset2,
                command: Port::new(0xA0),
                data: Port::new(0xA1),
            }],
        }
    }

    pub unsafe fn init(&mut self) {
        let mut wait_port: Port<u8> = Port::new(0x80); //The port to send interrupts on.
        let mut wait = || wait_port.write(0); //Simple closure

        //Let the PICS know we're gonna send a three-byte init sequence on the data port
        self.pics[0].command.write(CMD_INIT);
        wait();
        self.pics[1].command.write(CMD_INIT);
        wait();

        //Byte 1
        self.pics[0].data.write(self.pics[0].offset);
        wait();
        self.pics[1].data.write(self.pics[1].offset);
        wait();

        //Byte 2
        self.pics[0].data.write(4);
        wait();
        self.pics[1].data.write(2);
        wait();

        //Byte 3: set the mode
        self.pics[0].data.write(MODE_8086);
        wait();
        self.pics[1].data.write(MODE_8086);
    }

    pub fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        self.pics.iter().any(|p| p.handles_interrupt(interrupt_id))
    } //Cycle through the PICS until we find one that can handle this interrupt
    
    pub unsafe fn notify_end_of_interrupt(&mut self, interrupt_id: u8) {
        if self.handles_interrupt(interrupt_id) {
            if self.pics[1].handles_interrupt(interrupt_id) {
                self.pics[1].end_of_interrupt(); //If the interrupt was handled we can tell the handler to stop.
            }

            self.pics[0].end_of_interrupt();
        }   
    }
}
