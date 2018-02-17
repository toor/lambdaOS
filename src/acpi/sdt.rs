use core::mem;

#[derive(Copy, Clone, Debug)]
pub struct SdtHeader {
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [u8; 6],
    oem_table_id: [u8; 8],
    oem_revision: u32,
    creator_id: u32,
    creator_rev: u32,
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
