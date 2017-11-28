#[allow(non_uppercase_globals)]

use io::cpuio::Port;
use alloc::Vec;

const device_count: u8 = 4;
const bytes_per_sect: u16 = 512;

//Maximum number of sectors we can cache.
const max_cached_sectors: u16 = 2048;

const success: u8 = 0;
const eof: i8 = -1;
const failure: i8 = -2;

const dev_names: [&str; 26] = ["hda", "hdb", 
"hdc", "hdd", "hde", "hdf", "hdg",
"hdh", "hdi", "hdj", "hdk", "hdl",
"hdm", "hdn", "hdo", "hdp", "hdq", 
"hdr", "hds", "hdt", "hdu", "hdv",
"hdw", "hdx", "hdy", "hdz"];

const ata_ports: [u16; 4] = [0x1f0, 0x1f0, 0x170, 0x170];

lazy_static! {
    pub static ref devices: Vec<AtaDevice> = Vec::new();
}

pub struct CachedSector {
    cache: u8,
    sector: u64,
    status: u32,
}

///Represents an ATA PIO Mode disk, including registers used for controlling the disk across MMIO.
pub struct AtaDevice {
    //http://wiki.osdev.org/ATA_PIO_Mode#Registers
    pub master: u8,
    pub identity: [u16; 256],
    pub data_port: Port<u16>,
    pub error_port: Port<u16>,
    pub sector_count_port: Port<u16>,
    pub lba_low_port: Port<u16>,
    pub lba_mid_port: Port<u16>,
    pub lba_hi_port: Port<u16>,
    pub device_port: Port<u16>,
    pub command_port: Port<u8>,
    pub control_port: Port<u16>,
    pub sector_count: u64,
    pub bytes_per_sector: u16,
    pub cache: CachedSector,
}

impl AtaDevice {
    ///Returns a fully initialized ATA device if succesful, or None if some data could not be
    ///retrieved.
    pub fn new(&self, port_base: u16, master: u8) ->  Option<AtaDevice> {
        //Retrieve identity data.
        let mut dev = unsafe {
            AtaDevice {
                master: master,
                identity: [0; 256],
                data_port: Port::new(port_base),
                error_port: Port::new(port_base + 0x01),
                sector_count_port: Port::new(port_base + 0x02),
                lba_low_port: Port::new(port_base + 0x03),
                lba_mid_port: Port::new(port_base + 0x04),
                lba_hi_port: Port::new(port_base + 0x05),
                device_port: Port::new(port_base + 0x06),
                command_port: Port::new(port_base + 0x07),
                control_port: Port::new(port_base + 0x206),
                sector_count: 0,
                bytes_per_sector: 512,
                cache: CachedSector {
                    cache: 0,
                    sector: 0,
                    status: 0,
                },
            }
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

        //Read status off the command port.
        if dev.command_port.read() == 0 {
            println!("No device found");
            return None;
        } else {
            let mut timeout: u32 = 0;
            while (dev.command_port.read() & 0b10000000) == 1 {
                timeout += 1;
                if timeout == 100000 {
                    //Device timed out, throw errors.
                    println!("\nATA error: Drive detection timed out.");
                    println!("\nSkipping drive!");
                }
            }
        }

        //Check for non-standard ATAPI.
        if (dev.lba_mid_port.read() == 1) || (dev.lba_hi_port.read() == 1) {
            println!("Non-standard ATAPI, ignoring.");
            return None;
        }

        for timeout in 0..100000 {
            let status: u8 = dev.command_port.read();
            
            if (status & 0b00000001) == 1 {
                println!("Error occured.");
                return None;
            } else if (status & 0b00001000) == 1 {
                println!("Storing IDENTITY info.");
                for i in 0..255 {
                    dev.identity[i] = dev.data_port.read();
                }
                
                dev.sector_count = dev.identity[100] as u64;
                println!("Device successfully identified.");
                return Some(dev);
            }
        }

        //No device.
        println!("ATA error: Device detection timed out");
        println!("Skipping drive!");

        None
    }
}
