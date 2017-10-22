use core::marker::PhantomData;

//Begin init of the PIC chip
const CMD_INIT: u8 = 0x11;

//Interrupt acknowledgement
const CMD_END_OF_INTERRUPT: u8 = 0x20;

//PIC mode
const MODE_8086: u8 = 0x01;

#[macro_use]
pub mod cpuio;
pub mod serial;
use self::cpuio::Port;

//See http://wiki.osdev.org/PIC#Programming_with_the_8259_PIC for information on where this
//structure comes from.
struct Pic {
    offset: u8,
    command: Port<u8>,
    data: Port<u8>,
}

//Commands for a simple PIC, to check whether it handles the interrupt and to tell it that an
//interrupt has ended.
impl Pic {
    fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        self.offset <= interrupt_id && interrupt_id < self.offset + 8
    }

    unsafe fn end_of_interrupt(&mut self) {
        self.command.write(CMD_END_OF_INTERRUPT);
    }
}

//An array of two linked PIC chips.
pub struct ChainedPics {
    pics: [Pic; 2],
}

impl ChainedPics {
    //Return an instance of two PICS, Master and slave. Note that this function does not link the
    //PICS, this is done by the init function below.
    pub const unsafe fn new(offset1: u8, offset2: u8) -> ChainedPics {
        ChainedPics {
            pics: [
                Pic {
                    //The data port has an offset of 1 from the command ports of both the Master and
                    //Slave PICS.
                    offset: offset1,
                    command: Port::new(0x20),
                    data: Port::new(0x21),
                },
                Pic {
                    offset: offset2,
                    command: Port::new(0xA0),
                    data: Port::new(0xA1),
                },
            ],
        }
    }

    //Remap the IRQs and setup the 8259 PIC.
    pub unsafe fn init(&mut self) {
        //Write garbage data to a port as a method of telling the CPU to wait for a bit in-between
        //commands.
        let mut wait_port: Port<u8> = Port::new(0x80);
        let mut wait = || wait_port.write(0);

        //Send each PIC the 0x11 byte to tell them to expect initialization
        self.pics[0].command.write(CMD_INIT);
        wait();
        self.pics[1].command.write(CMD_INIT);
        wait();

        //Master PIC Vector offset.
        self.pics[0].data.write(self.pics[0].offset);
        wait();
        //Slave PIC Vector offset.
        self.pics[1].data.write(self.pics[1].offset);
        wait();

        //Tell the Master PIC there is a slave PIC at IRQ 2.
        self.pics[0].data.write(4);
        wait();
        //Tell the Slave PIC its cascade identity (IRQ 2)
        self.pics[1].data.write(2);
        wait();

        //Byte 3: set the mode
        self.pics[0].data.write(MODE_8086);
        wait();
        self.pics[1].data.write(MODE_8086);
    }

    //Cycle through the PICS until we find one that can handle this interrupt.
    pub fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        self.pics.iter().any(|p| p.handles_interrupt(interrupt_id))
    }

    //Write magic EOI command.
    pub unsafe fn notify_end_of_interrupt(&mut self, interrupt_id: u8) {
        if self.handles_interrupt(interrupt_id) {
            //If the Slave can handle this interrupt, tell it the interrupt has ended.
            if self.pics[1].handles_interrupt(interrupt_id) {
                self.pics[1].end_of_interrupt();
            }

            //Notify the Master PIC that the interrupt has ended.
            self.pics[0].end_of_interrupt();
        }
    }
}
