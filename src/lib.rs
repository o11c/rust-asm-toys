#![crate_name = "asm"]

#![feature(asm)]
#![no_std]

mod syscall_asm;
mod syscall_nr;

pub mod syscall;
