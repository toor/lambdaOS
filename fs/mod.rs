pub const SEARCH_FAILURE: u64 = 0xffffffffffffffff;
pub const ROOT_ID: u64 = 0xffffffffffffffff;
pub const ENTRIES_PER_BLOCK: u8 = 2;
pub const FILENAME_LEN: u8 = 218;
pub const RESERVED_BLOCKS: u8 = 16;
pub const BYTES_PER_BLOCK: u16 = 512;
pub const FILE_TYPE: u8 = 0;
pub const DIRECTORY_TYPE: u8 = 1;
pub const DELETED_ENTRY: u64 = 0xfffffffffffffffe;
pub const RESERVED_BLOCK: u64 = 0xfffffffffffffff0;
pub const END_OF_CHAIN: u64 = 0xffffffffffffffff;

///Represents a file entry.
#[repr(C, packed)]
pub struct Entry {
    parent_id: u64,
    ftype: u8,
    name: [char; FILENAME_LEN],
    perms: u8,
    owner: u16,
    group: u16,
    hundredths: u8,
    seconds: u8,
    minutes: u8,
    hours: u8,
    day: u8,
    month: u8,
    year: u16,
    payload: u64,
    size: u64,
}
