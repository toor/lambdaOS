use core::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT};

pub static AHCI_BASE: AtomicUsize = ATOMIC_USIZE_INIT;
