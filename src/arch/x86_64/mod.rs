//! Architecture-specific code for AMD64.

pub mod interrupts;
pub mod memory;
pub mod init;

pub use self::init::init;
