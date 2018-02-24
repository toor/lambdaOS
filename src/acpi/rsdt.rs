use super::sdt::SdtHeader;
use core::mem;
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
                let array = unsafe { mem::transmute::<&[u8], &[u32]>(sdt.data()) };

                Rsdt {
                    sdt: sdt,
                    other_entries: &array,
                }
            },
            _ => panic!("Non-matching signature, aborting ..."),
        }
    }

    pub fn find_sdt(&self, signature: &[u8]) -> Option<TableType> {
        // Iterate over all the pointers to other tables.
        for i in 0 .. self.other_entries.len() {
            let sdt = self.other_entries[i] as usize as *const SdtHeader;
            let sdt = unsafe { &*sdt };

            let sig = &sdt.signature;

            match sig {
                b"APIC" if sig == signature => return Some(TableType::Madt(Madt::new(sdt))),
                _ => return None,
            }
        }

        None
    }
}

pub enum TableType {
    Madt(super::madt::Madt),
}
