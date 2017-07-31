#![allow(unused_features, dead_code, unused_variables)]
#![feature(const_fn, prelude_import, test, raw, ptr_as_ref,
           core_prelude, core_slice_ext, libc, unique, asm)]
#![no_std]

extern crate spin;

mod syscall;

#[no_mangle]
pub extern fn sys_time() -> u64 {
    unsafe { syscall::syscall0(201) }
}

#[no_mangle]
pub fn sys_test() -> u64 {
    unsafe { syscall::syscall6(16, 32, 64, 128, 256, 512, 1024) }
}