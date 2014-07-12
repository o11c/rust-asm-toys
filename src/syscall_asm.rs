#![allow(dead_code)]

#[inline(always)]
pub unsafe fn syscall6(nr: int, p1: int, p2: int, p3: int, p4: int, p5: int, p6: int) -> int
{
    let mut rv : int;
    asm!("syscall"
         : "={rax}"(rv)
         : "{rax}"(nr), "{rdi}"(p1), "{rsi}"(p2), "{rdx}"(p3), "{r10}"(p4), "{r8}"(p5), "{r9}"(p6)
         : "%rcx", "%r11", "memory");
    rv
}

#[inline(always)]
pub unsafe fn syscall5(nr: int, p1: int, p2: int, p3: int, p4: int, p5: int) -> int
{
    let mut rv : int;
    asm!("syscall"
         : "={rax}"(rv)
         : "{rax}"(nr), "{rdi}"(p1), "{rsi}"(p2), "{rdx}"(p3), "{r10}"(p4), "{r8}"(p5)
         : "%rcx", "%r11", "memory");
    rv
}

#[inline(always)]
pub unsafe fn syscall4(nr: int, p1: int, p2: int, p3: int, p4: int) -> int
{
    let mut rv : int;
    asm!("syscall"
         : "={rax}"(rv)
         : "{rax}"(nr), "{rdi}"(p1), "{rsi}"(p2), "{rdx}"(p3), "{r10}"(p4)
         : "%rcx", "%r11", "memory");
    rv
}

#[inline(always)]
pub unsafe fn syscall3(nr: int, p1: int, p2: int, p3: int) -> int
{
    let mut rv : int;
    asm!("syscall"
         : "={rax}"(rv)
         : "{rax}"(nr), "{rdi}"(p1), "{rsi}"(p2), "{rdx}"(p3)
         : "%rcx", "%r11", "memory");
    rv
}

#[inline(always)]
pub unsafe fn syscall2(nr: int, p1: int, p2: int) -> int
{
    let mut rv : int;
    asm!("syscall"
         : "={rax}"(rv)
         : "{rax}"(nr), "{rdi}"(p1), "{rsi}"(p2)
         : "%rcx", "%r11", "memory");
    rv
}

#[inline(always)]
pub unsafe fn syscall1(nr: int, p1: int) -> int
{
    let mut rv : int;
    asm!("syscall"
         : "={rax}"(rv)
         : "{rax}"(nr), "{rdi}"(p1)
         : "%rcx", "%r11", "memory");
    rv
}

#[inline(always)]
pub unsafe fn syscall0(nr: int) -> int
{
    let mut rv : int;
    asm!("syscall"
         : "={rax}"(rv)
         : "{rax}"(nr)
         : "%rcx", "%r11", "memory");
    rv
}
