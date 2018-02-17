use core::mem;

#[derive(Copy, Clone, Debug)]
pub struct SdtHeader {
    pub signature: [u8; 4],
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub oem_table_id: [u8; 8],
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_rev: u32,
}

impl SdtHeader {
    /// Check if this is valid.
    pub fn checksum(&self) -> bool {
        let mut sum: u8 = 0;

        for i in 0..self.length {
            let slice: [u8; mem::size_of::<Self>()] = unsafe { mem::transmute_copy(self) };

            sum = slice.iter().sum();
        }

        sum == 0
    }
}
