pub mod process;

pub use self::process::*;

#[inline(never)]
pub fn match_syscall(num: u64, a: u64, b: u64, c: u64, d: u64, e: u64, f: u64) -> u64 {
    match num {
        // sys_execve
        11 => {
            let res = match process::create(test_syscall_proc, "test_syscall_proc") {
                // Success.
                Ok(_) => 0,
                // Failure - some error code.
                Err(e) => e,
            };
            
            res
        }
    }
}
