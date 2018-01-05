#[inline(always)]
#[allow(unused_mut)]
pub unsafe fn syscall0(arg0: u64) -> u64 {
    let mut ret: u64;
    asm!("int 0x80" : "={rax}" (ret) : "{rax}" (arg0) : "rcx", "r11", "memory" : "intel", "volatile");
    ret
}

#[inline(always)]
#[allow(unused_mut)]
pub unsafe fn syscall1(arg0: u64, arg1: u64) -> u64 {
    let mut ret: u64;
    asm!("int 0x80" : "={rax}" (ret) : "{rax}" (arg0), "{rdi}" (arg1)
                   : "rcx", "r11", "memory" : "intel", "volatile");
    ret
}

#[inline(always)]
#[allow(unused_mut)]
pub unsafe fn syscall2(arg0: u64, arg1: u64, arg2: u64) -> u64 {
    let mut ret: u64;
    asm!("int 0x80" : "={rax}" (ret) : "{rax}" (arg0), "{rdi}" (arg1), "{rsi}" (arg2)
                   : "rcx", "r11", "memory" : "intel", "volatile");
    ret
}

#[inline(always)]
#[allow(unused_mut)]
pub unsafe fn syscall3(arg0: u64, arg1: u64, arg2: u64, arg3: u64) -> u64 {
    let mut ret: u64;
    asm!("int 0x80" : "={rax}" (ret) : "{rax}" (arg0), "{rdi}" (arg1), "{rsi}" (arg2), "{rdx}" (arg3)
                   : "rcx", "r11", "memory" : "intel", "volatile");
    ret
}

#[inline(always)]
#[allow(unused_mut)]
pub unsafe fn syscall4(arg0: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64) -> u64 {
    let mut ret: u64;
    asm!("int 0x80" : "={rax}" (ret)
                   : "{rax}"  (arg0), "{rdi}"  (arg1), "{rsi}"  (arg2), "{rdx}"  (arg3), "{r10}"  (arg4)
                   : "rcx", "r11", "memory" : "intel", "volatile");
    ret
}

#[inline(always)]
#[allow(unused_mut)]
pub unsafe fn syscall5(arg0: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64) -> u64 {
    let mut ret: u64;
    asm!("int 0x80" : "={rax}" (ret)
                   : "{rax}" (arg0), "{rdi}" (arg1), "{rsi}" (arg2), "{rdx}" (arg3), "{r10}" (arg4), "{r8}" (arg5)
                   : "rcx", "r11", "memory"
                   : "intel", "volatile");
    ret
}

#[inline(always)]
#[allow(unused_mut)]
pub unsafe fn syscall6(arg0: u64,
                       arg1: u64,
                       arg2: u64,
                       arg3: u64,
                       arg4: u64,
                       arg5: u64,
                       arg6: u64)
                       -> u64 {
    let mut ret: u64;
    asm!("int 0x80" : "={rax}" (ret)
                   : "{rax}" (arg0), "{rdi}" (arg1), "{rsi}" (arg2), "{rdx}" (arg3),
                     "{r10}" (arg4), "{r8}" (arg5), "{r9}" (arg6)
                   : "rcx", "r11", "memory"
                   : "intel", "volatile");
    ret
}
