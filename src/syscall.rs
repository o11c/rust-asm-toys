#[allow(unused_imports)]
use syscall_asm::{syscall0,syscall1,syscall2,syscall3,syscall4,syscall5,syscall6};

use syscall_nr;

pub enum Maybe
{
    Yes(uint),
    No(uint),
}

#[no_split_stack]
pub unsafe fn write(fd: i32, buf: *const u8, len: uint) -> Maybe
{
    let rv = syscall3(syscall_nr::write, fd as int, buf as int, len as int);
    if -4096 <= rv && rv < 0
    {
        No(-rv as uint)
    }
    else
    {
        Yes(rv as uint)
    }
}

#[no_split_stack]
pub unsafe fn exit(status: i32)
{
    syscall1(syscall_nr::exit, status as int);
}
