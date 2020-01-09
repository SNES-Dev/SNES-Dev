#![no_std]
#![no_main]
#![allow(non_upper_case_globals)]
#![feature(asm)]
#![feature(core_intrinsics)]

#[cfg(not(target="wc65c816"))]
compile_error!("Can only build snes-dev rust for the 65816 architecture");





pub mod variables;
pub mod volatile;
pub mod faults;
pub mod dma;
pub mod pointer;

#[cfg(feature="memctl")]
pub mod memctl;

