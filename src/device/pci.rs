use device::io::Port;

pub struct Pci {
    pub cfg_address: Port<u32>,
    pub cfg_data: Port<u32>,
}

impl Pci {
    pub unsafe fn read_config(&mut self, bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
        let address: u32 = 0x80000000 | (bus as u32) << 16 | (slot as u32) << 11 | 
            (function as u32) << 8 |
            (offset & 0xFC) as u32;
        
        self.cfg_address.write(address);
        self.cfg_data.read()
    }

    pub unsafe fn probe(&mut self, bus: u8, slot: u8, function: u8) {}
}

#[derive(Debug, Copy, Clone)]
pub struct Device {
    pub device_id: u16,
    pub vendor_id: u16,
    pub rev_id: u8,
    pub subclass: u8,
    //TODO: device class
    /// Whether this device is multifunction or not.
    multifunction: bool,
    /// Base addresses.
    bars: [u32; 6],
}

impl Device {
    
}
