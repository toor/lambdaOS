use device::io::Port;
use spin::Mutex;
use alloc::Vec;
use core::fmt;
// use core::num::Float;

#[allow(dead_code)]
const MAX_BUS: u8 = 255;

#[allow(dead_code)]
const MAX_DEVICE: u8 = 31;

#[allow(dead_code)]
const MAX_FUNCTION: u8 = 7;

static PCI: Mutex<Pci> = Mutex::new(Pci {
    cfg_address: unsafe { Port::new(0xCF8) },
    cfg_data: unsafe { Port::new(0xCFC) },
});

lazy_static! {
    static ref DEVICES: Mutex<Vec<Device>> = Mutex::new(Vec::new());
}

pub struct Pci {
    pub cfg_address: Port<u32>,
    pub cfg_data: Port<u32>,
}

impl Pci {
    /// Read an aligned dword from the PCI configuration space.
    pub unsafe fn read_config(&mut self, bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
        let address: u32 = 0x80000000 | (bus as u32) << 16 | (slot as u32) << 11
            | (func as u32) << 8 | (offset & 0xFC) as u32;

        self.cfg_address.write(address);
        self.cfg_data.read()
    }

    /// Read data from `CFG_DATA` to determine unique info about a device.
    pub unsafe fn probe(&mut self, bus: u8, slot: u8, function: u8) -> Option<Device> {
        let config_0 = self.read_config(bus, slot, function, 0);

        if config_0 == 0xFFFFFFFF {
            return None;
        }

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
            class: DeviceClass::from_u8((config_4 >> 24) as u8),
            multifunction: config_c & 0x800000 != 0,
            bars: [0; 6],
        })
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}: 0x{:04x} 0x{:04x} {:?} {:02x}",
            self.bus,
            self.device,
            self.function,
            self.vendor_id,
            self.device_id,
            self.class,
            self.subclass
        )
    }
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
    /// Convert a given device code to a possible `DeviceClass` variant.
    fn from_u8(c: u8) -> Self {
        if c <= DeviceClass::DataAndSignalProcessing as u8 {
            unsafe { ::core::mem::transmute(c) }
        } else {
            DeviceClass::Unknown
        }
    }
}

/// A PCI device.
#[derive(Debug, Copy, Clone)]
pub struct Device {
    bus: u8,
    function: u8,
    device: u8,
    device_id: u16,
    vendor_id: u16,
    rev_id: u8,
    subclass: u8,
    class: DeviceClass,
    /// Whether this device is multifunction or not.
    multifunction: bool,
    /// Base addresses.
    bars: [u32; 6],
}

impl Device {
    fn address(&self, offset: u32) -> u32 {
        return 1 << 31 | (self.bus as u32) << 16 | (self.device as u32) << 11
            | (self.function as u32) << 8 | (offset as u32 & 0xFC);
    }

    /// Read.
    pub unsafe fn read(&self, offset: u32) -> u32 {
        let address = self.address(offset);
        PCI.lock().cfg_address.write(address);
        return PCI.lock().cfg_data.read();
    }

    /// Write.
    pub unsafe fn write(&self, offset: u32, value: u32) {
        let address = self.address(offset);
        PCI.lock().cfg_address.write(address);
        PCI.lock().cfg_data.write(value);
    }

    /// Set a certain flag
    pub unsafe fn set_flag(&self, offset: u32, flag: u32, toggle: bool) {
        let mut value = self.read(offset);

        if toggle {
            value |= flag
        } else {
            value &= 0xFFFFFFFF - flag;
        }
        self.write(offset, value);
    }

    unsafe fn load_bars(&mut self) {
        for i in 0..6 {
            let bar = self.read(i * 4 + 0x10);
            if bar > 0 {
                self.bars[i as usize] = bar;
                self.write(i * 4 + 0x10, 0xFFFFFFFF);
                let size = (0xFFFFFFFF - (self.read(i * 4 + 0x10) & 0xFFFFFFF0)) + 1;
                self.write(i * 4 + 0x10, bar);
                if size > 0 {
                    self.bars[i as usize] = size;
                }
            }
        }
    }

    pub fn bar(&self, index: usize) -> u32 {
        self.bars[index]
    }
}

fn init_dev(bus: u8, dev: u8) {
    for func in 0..MAX_FUNCTION {
        unsafe {
            let device = PCI.lock().probe(bus, dev, func);

            match device {
                // Device found, load bars.
                Some(mut d) => {
                    d.load_bars();
                    DEVICES.lock().push(d);
                }

                None => {}
            }
        }
    }
}

fn init_bus(bus: u8) {
    for dev in 0..MAX_DEVICE {
        init_dev(bus, dev);
    }
}

pub fn init() {
    for bus in 0..MAX_BUS {
        init_bus(bus);
    }

    println!("[ dev ] Discovered {} PCI devices.", DEVICES.lock().len());

    for dev in DEVICES.lock().iter_mut() {
        // Check the type of device, in order to identify important stuff that we will use.
        match dev.class {
            DeviceClass::Legacy => {}
            DeviceClass::MassStorage => {
                match dev.subclass {
                    0x06 => {
                        use device::ahci::hba::AHCI_BASE;
                        use core::sync::atomic::Ordering;

                        // Read header offset 24h to get reference to the ABAR.
                        let mut bar = unsafe { dev.read(0x24) };

                        // Read bits 31-34, these point to the ABAR.
                        let address = bar & 0xFFFFFFF0;

                        AHCI_BASE.store(address as usize, Ordering::SeqCst);

                        println!(
                            "[ dev ] Found AHCI controller. Controller mapped at {:#x}",
                            address
                        );
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
