#![no_std]
#![no_main]

#![feature(globs)]
#![feature(intrinsics)]
#![feature(lang_items)]

extern crate asm;

extern crate core;
extern crate rlibc;

use asm::syscall;

use core::prelude::*;


extern "rust-intrinsic" { fn abort() -> !; }

#[lang="stack_exhausted"]
unsafe fn x_stack_exhausted()
{
    abort();
}

#[lang="eh_personality"]
unsafe fn x_eh_personality()
{
    abort();
}

#[lang="begin_unwind"]
unsafe fn x_begin_unwind()
{
    abort();
}


#[no_mangle]
pub extern "C" fn _start()
{
    let s = "Hello, World!\n".as_bytes();
    unsafe
    {
        syscall::write(0, s.as_ptr(), s.len());
        syscall::exit(0);
    }
}
