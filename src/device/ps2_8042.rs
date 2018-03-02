use spin::Mutex;
use device::io::Port;

pub struct Ps2 {
    pub controller: Port<u8>,
    pub device: Port<u8>,
}

impl Ps2 {
    pub const unsafe fn new(controller: u16, device: u16) -> Ps2 {
        Ps2 {
            controller: Port::new(controller),
            device: Port::new(device),
        }
    }

    /// Poll bit 0 of status register: "Output buffer empty/full"
    pub fn wait_then_read(&mut self) -> u8 {
        while self.controller.read() & 0x1 == 0 {}
        self.device.read()
    }

    /// Poll bit 1 of status register: "Input buffer empty/full"
    pub fn wait_then_write(&mut self, data: u8) {
        while self.controller.read() & 0x2 == 1 {}
        self.device.write(data);
    }

    pub fn init(&mut self) {
        println!("[ dev ] Initialising PS/2 8042 controller.");
        // Disable devices.
        self.controller.write(0xAD);
        self.controller.write(0xA7);

        // Flush output buffer.
        self.device.read();

        // Setup Controller Config Byte.
        self.controller.write(0x20);
        let mut config_byte: u8 = self.wait_then_read();

        // Disable IRQs.
        config_byte &= !(1 << 0);
        config_byte &= !(1 << 1);

        // Write back the modified config.
        self.controller.write(0x60);
        self.wait_then_write(config_byte);

        // Controller self test.
        self.controller.write(0xAA);
        assert!(self.wait_then_read() == 0x55, "PS/2 self test failed");

        // Interface tests.
        self.controller.write(0xAB);
        assert!(self.wait_then_read() == 0x0, "Interface tests failed",);

        // Enable devices.
        self.controller.write(0xAE);

        // Config byte.
        self.controller.write(0x20);
        let mut enable: u8 = self.wait_then_read();

        // Re-enable IRQs.
        enable |= 1 << 0;

        self.controller.write(0x60);
        self.wait_then_write(enable);

        // Clear output buffer.
        self.device.read();

        println!("[ dev ] PS/2 8042 initialised.");
    }

    pub fn read_char(&mut self) -> u8 {
        self.device.read()
    }
}

pub static PS2: Mutex<Ps2> = Mutex::new(unsafe { Ps2::new(0x64, 0x60) });

pub fn read_char() -> u8 {
    PS2.lock().read_char()
}
