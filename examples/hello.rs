#![no_std]
#![no_main]

#![feature(globs)]

extern crate asm;

extern crate core;
extern crate rlibc;

use asm::syscall;

use core::prelude::*;


#[no_mangle] #[no_split_stack]
pub extern "C" fn _start2(_: *const int)
{
    let s = "Hello, World!\n".as_bytes();

    unsafe
    {
        syscall::write(1, s.as_ptr(), s.len());
        syscall::exit(0);
    }
}
