#![crate_name = "asm"]
#![no_std]

#![feature(asm)]
#![feature(intrinsics)]
#![feature(lang_items)]

mod syscall_asm;
mod syscall_nr;

pub mod syscall;


extern "rust-intrinsic" { fn abort() -> !; }

#[no_mangle] #[no_split_stack] #[lang="stack_exhausted"]
unsafe fn rust_stack_exhausted()
{
    abort();
}

#[no_split_stack] #[lang="eh_personality"]
unsafe fn x_eh_personality()
{
    abort();
}

#[no_split_stack] #[lang="begin_unwind"]
unsafe fn x_begin_unwind()
{
    abort();
}
