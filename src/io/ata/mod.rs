use io::cpuio;

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
    master: u8,
    identify: [u8; 256],
    data_port: cpuio::Port<u16>,
    error_port: cpuio::Port<u16>,
    sector_count_port: cpuio::Port<u16>,
    lba_low_port: cpuio::Port<u16>,
    lba_mid_port: cpuio::Port<u16>,
    lba_hi_port: cpuio::Port<u16>,
    device_port: cpuio::Port<u16>,
    command_port: cpuio::Port<u16>,
    control_port: cpuio::Port<u16>,
    exists: u8,
    sector_count: u64,
    bytes_per_sector: u16,
    cache: &CachedSector,
}

impl AtaDevice {
    pub fn new(&self) -> AtaDevice {
        AtaDevice {
            
        }
    }
}
