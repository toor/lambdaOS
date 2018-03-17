use super::sdt::SdtHeader;
use core::slice;

use super::madt::Madt;

#[derive(Debug)]
pub struct Rsdt<'a> {
    pub sdt: &'static SdtHeader,
    pub other_entries: &'a [u32],
}

impl<'a> Rsdt<'a> {
    pub fn new(sdt: &'static SdtHeader) -> Self {
        match &sdt.signature {
            b"RSDT" => {
                let array = Rsdt::data(sdt);

                Rsdt {
                    sdt: sdt,
                    other_entries: array,
                }
            }
            _ => panic!("Non-matching signature, aborting ..."),
        }
    }

    /// Retrieve a pointed-to table using a byte signature.
    pub fn find_sdt(&self, signature: &[u8]) -> Option<TableType> {
        // Iterate over all the pointers to other tables.
        for i in self.other_entries.iter() {
            let sdt = *i as *const SdtHeader;
            let sdt = unsafe { &*sdt };

            let sig: &[u8] = &sdt.signature;

            if sig != signature {
                continue;
            } else {
                match signature {
                    // TODO: Support more tables.
                    b"APIC" => return Some(TableType::Madt(Madt::new(sdt))),
                    _ => return None,
                }
            }
        }

        None
    }

    /// Return RSDT data.
    pub fn data(sdt: &'static SdtHeader) -> &[u32] {
        // len - sizeof(header) / 4.
        unsafe { slice::from_raw_parts(sdt.data_address() as *const u32, sdt.data_len() / 4) }
    }
}

pub enum TableType {
    Madt(Madt),
    Facp,
    Hpet,
}
