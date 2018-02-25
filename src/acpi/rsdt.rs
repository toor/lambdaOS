use super::sdt::SdtHeader;
use core::mem;
use core::str;
use super::madt::Madt;

#[derive(Debug)]
pub struct Rsdt<'a> {
    pub sdt: &'static SdtHeader,
    pub other_entries: &'a[u32],
}

impl <'a> Rsdt <'a> {
    pub fn new(sdt: &'static SdtHeader) -> Self {
        match &sdt.signature {
            b"RSDT" => {
                let array = unsafe { sdt.data() };

                Rsdt {
                    sdt: sdt,
                    other_entries: &array,
                }
            },
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
                    b"APIC" => return Some(TableType::Madt(Madt::new(sdt))),
                    _ => return None,
                }
            }
        }

        None
    }
}

pub enum TableType {
    Madt(Madt),
    Facp,
    Hpet,
}
