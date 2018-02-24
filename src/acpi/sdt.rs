use core::mem;
use core::slice;

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
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
    
    /// Return the starting address of this table's data.
    pub fn data_address(&self) -> usize {
        self as *const _ as usize + mem::size_of::<Self>()
    }

    /// Return the length of this table's data.
    pub fn data_len(&self) -> usize {
        let total_size = self.length as usize;
        let header_size = mem::size_of::<Self>();
        
        // Check if the length is bigger than the header itself. If it is, other data exists.
        if total_size >= header_size {
            return total_size - header_size;
        } else {
            0 // No extra data.
        }
    }

    /// Return a slice of this table's data.
    pub unsafe fn data(&self) -> &[u8] {
        slice::from_raw_parts(self.data_address() as *const u8, self.data_len())
    }
}
