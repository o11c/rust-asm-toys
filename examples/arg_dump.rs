#![no_std]
#![no_main]

#![feature(globs)]

extern crate asm;

extern crate core;
extern crate rlibc;

use asm::syscall;

use core::prelude::*;

#[no_split_stack]
unsafe fn strlen(mut s: *const u8) -> uint
{
    let mut len: uint = 0;
    while *s != 0
    {
        s = s.offset(1);
        len += 1;
    }
    len
}

#[no_split_stack]
unsafe fn write_cstr(s: *const u8)
{
    syscall::write(1, s, strlen(s));
}

#[no_split_stack]
unsafe fn write_str(s: &str)
{
    let s = s.as_bytes();
    syscall::write(1, s.as_ptr(), s.len());
}

#[no_mangle] #[no_split_stack]
pub extern "C" fn _start2(rsp: *const int)
{
    unsafe
    {
        let argc = *rsp;
        let argv: *const *const u8 = core::mem::transmute(rsp.offset(1));
        let envp = argv.offset(argc + 1);

        write_str("Args:");
        let mut i = 0;
        while i < argc
        {
            write_str("\n  - ");
            write_cstr(*argv.offset(i));
            i += 1;
        }
        write_str("\n\n");

        write_str("Environment:");
        i = 0;
        while (*envp.offset(i)).is_not_null()
        {
            write_str("\n  - ");
            write_cstr(*envp.offset(i));
            i += 1;
        }
        write_str("\n\n");

        syscall::exit(0);
    }
}
