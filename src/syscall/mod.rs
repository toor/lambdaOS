pub mod process;

pub use self::process::*;

#[inline(never)]
pub fn match_syscall(num: u64, a: u64, b: u64, c: u64, d: u64, e: u64, f: u64) -> u64 {
    match num {
        // sys_execve
        /*11 => {
            let res = match process::create(test_syscall_proc, "test_syscall_proc") {
                // Success.
                Ok(_) => 0,
                // Failure - some error code.
                Err(e) => e,
            };
            
            res
        }*/
        16 => test(a, b, c, d, e, f),
        _ => err(num, a, b, c, d, e, f),
    }
}

fn test(a: u64, b: u64, c: u64, d: u64, e: u64, f: u64) -> u64 {
    a + b + c + d + e + f
}

fn err(num: u64, _a:u64, _b:u64, _c:u64, _d:u64, _e:u64, _f:u64) -> u64 {
    println!("Unknown syscall of type {}", num);
    0
}
