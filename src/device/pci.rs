use device::io::Port;
use spin::Mutex;

static PCI: Mutex<Pci> = Mutex::new(Pci {
    cfg_address: unsafe { Port::new(0xCF8) },
    cfg_data: unsafe { Port::new(0xCFC) },
});

pub struct Pci {
    pub cfg_address: Port<u32>,
    pub cfg_data: Port<u32>,
}

impl Pci {
    pub unsafe fn read_config(&mut self, bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
        let address: u32 = 0x80000000 | (bus as u32) << 16 | (slot as u32) << 11 | 
            (func as u32) << 8 |
            (offset & 0xFC) as u32;
        
        self.cfg_address.write(address);
        self.cfg_data.read()
    }
    
    /// Read data from `CFG_DATA` to determine unique info about a device.
    pub unsafe fn probe(&mut self, bus: u8, slot: u8, function: u8) -> Option<Device> {
        let config_0 = self.read_config(bus, slot, function, 0);

        if config_0 == 0xFFFFFFFF {
            return None;
        }

        println!("Found device {}-{}-{}", bus, slot, function);

        let config_4 = self.read_config(bus, slot, function, 0x8);
        let config_c = self.read_config(bus, slot, function, 0xC);

        Some(Device {
            bus: bus,
            function: function,
            device: slot,
            device_id: (config_0 >> 16) as u16,
            vendor_id: config_0 as u16,
            rev_id: config_4 as u8,
            subclass: (config_4 >> 16) as u8,
            class: DeviceClass::from((config_4 >> 24)),
            multifunction: config_c & 0x800000 != 0,
            bars: [0; 6],
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Device {
    bus: u8,
    function: u8,
    device: u8,
    pub device_id: u16,
    pub vendor_id: u16,
    pub rev_id: u8,
    pub subclass: u8,
    class: DeviceClass,
    /// Whether this device is multifunction or not.
    multifunction: bool,
    /// Base addresses.
    bars: [u32; 6],
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
#[allow(dead_code)]
pub enum DeviceClass {
    Legacy = 0x00,
    MassStorage = 0x01,
    Network = 0x02,
    Display = 0x03,
    Multimedia = 0x04,
    Memory = 0x05,
    BridgeDevice = 0x06,
    SimpleCommunication = 0x07,
    BaseSystemPeripheral = 0x08,
    InputDevice = 0x09,
    DockingStation = 0x0A,
    Processor = 0x0B,
    SerialBus = 0x0C,
    Wireless = 0x0D,
    IntelligentIO = 0x0E,
    SatelliteCommunication = 0x0F,
    EncryptionDecryption = 0x10,
    DataAndSignalProcessing = 0x11,
    Unknown,
}

impl DeviceClass {
    fn from_u8(c: u8) -> Self {
        if c <= DeviceClass::DataAndSignalProcessing as u8 {
            unsafe { ::core::mem::transmute(c) }
        } else {
            DeviceClass::Unknown
        }
    }
}

impl Device {
    fn address(&self, offset: u32) -> u32 {
        return 1 << 31 | (self.bus as u32) << 16 | (self.device as u32) << 11 |
            (self.function as u32) << 8 | (offset as u32 & 0xFC);
    }

    /// Read.
    pub unsafe fn read(&self, offset: u32) -> u32 {
        let address = self.address(offset);
        PCI.lock().cfg_address.write(address);
        return PCI.lock().cfg_data.read();
    }
}
