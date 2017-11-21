use io::cpuio::Port;

const DEVICE_COUNT: u8 = 4;
const BYTES_PER_SECT: u16 = 512;

//Maximum number of sectors we can cache.
const MAX_CACHED_SECTORS: u16 = 2048;

const SUCCESS: u8 = 0;
const EOF: i8 = -1;
const FAILURE: i8 = -2;

const DEV_NAMES = ["hda", "hdb", 
"hdc", "hdd", "hde", "hdf", "hdg"
"hdh", "hdi", "hdj", "hdk", "hdl",
"hdm", "hdn", "hdo", "hdp", "hdq", 
"hdr", "hds", "hdt", "hdu", "hdv",
"hdw", "hdx", "hdy", "hdz"];

pub struct CachedSector {
    cache: &u8,
    sector: u64,
    status: u32,
}

pub struct AtaDevice {
    //http://wiki.osdev.org/ATA_PIO_Mode#Registers
    pub master: u8,
    pub identify: [u16; 256],
    pub data_port: Port<u16>,
    pub error_port: Port<u16>,
    pub sector_count_port: Port<u16>,
    pub lba_low_port: Port<u16>,
    pub lba_mid_port: Port<u16>,
    pub lba_hi_port: Port<u16>,
    pub device_port: Port<u16>,
    pub command_port: Port<u16>,
    pub control_port: Port<u16>,
    pub exists: u8,
    pub sector_count: u64,
    pub bytes_per_sector: u16,
    pub cache: &CachedSector,
}

impl AtaDevice {
    pub fn new(&self, port_base: u16, master: u8) -> AtaDevice {
        //Retrieve identity data.
        let mut dev = AtaDevice {
            master: master,
            identify: [0],
            data_port: port_base,
            error_port: Port::new(port_base + 0x01),
            sector_count_port: Port::new(port_base + 0x02),
            lba_low_port: Port::new(port_base + 0x03),
            lba_mid_port: Port::new(port_base + 0x04),
            lba_hi_port: Port::new(port_base + 0x05),
            device_port: Port::new(port_base + 0x06),
            command_port: Port::new(port_base + 0x07),
            dev.control_port: Port::new(port_base + 0x206),
            exists: 0,
            bytes_per_sector: 512,
            //TODO: Use kalloc to create some cache for the disk.
        };

        if dev.master == 1 {
            dev.device_port.write(0xa0);
        } else {
            //Slave device.
            dev.device_port.write(0xb0);
        }
        
        //Zero sec_count and lba ports.
        dev.sector_count_port.write(0);
        dev.lba_low_port.write(0);
        dev.lba_mid_port.write(0);
        dev.lba_hi_port.write(0);

        //IDENTIFY command.
        dev.command_port.write(0xEC);

        //Read boolean off the commnand port.
        if dev.command_port.read() == 0 {
            dev.exists = 0;
            println!("No device found");
            return dev;
        } else {
            let timeout: u32 = 0;
            while (dev.command_port.read() & 0b10000000) {
                if (timeout += 1) == 100000 {
                    
                }
            }
        }

        //Check for non-standard ATAPI.
        if (dev.lba_mid_port.read() == 1) || (dev.lba_hi_port.read() == 1) {
            dev.exists = 0;
            println!("Non-standard ATAPI, ignoring.");
        }

        for timeout in 0..100000 {
            let status: u8 = dev.command_port.read();
            
            if status & 0b00000001 {
                dev.exists = 0;
                println!("Error occured.");
                return dev;
            } else if status & 0b00001000 {
                println!("Storing IDENTITY info.");
                for i in 0..255 {
                    dev.identify[i] = dev.data_port.read();
                }
                
                dev.sector_count = dev.identity[100] as *mut u64;
                println!("Device successfully identified.");
                dev
            }
        }

        //No device.
        dev.exists = 0;
        println!("ATA error: Device detection timed out");
        println!("Skipping drive!");
    }
}
