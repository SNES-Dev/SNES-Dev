#![no_std]
#![no_main]
#![allow(non_upper_case_globals)]
#![feature(asm)]
#![feature(core_intrinsics)]

use core::sync::atomic::{compiler_fence, Ordering};
use crate::volatile::LockedVolatileCell;
#[cfg(not(target="wc65c816"))]
compile_error!("Can only build snes-dev rust for the 65816 architecture");

static mut _DISABLE_INTERRUPTS: u16 = 0;

pub struct LockInterrupts;

impl LockInterrupts{
	pub fn new() -> LockInterrupts{
		unsafe{
			asm!("SEI"::::"volatile");
			compiler_fence(Ordering::SeqCst);
			_DISABLE_INTERRUPTS += 1;
		}
		LockInterrupts
	}
}

impl Drop for LockInterrupts{
	fn drop(&mut self){
		compiler_fence(Ordering::SeqCst);
		unsafe {
			_DISABLE_INTERRUPTS-= 1;
			if _DISABLE_INTERRUPTS==0{
				asm!("CLI"::::"volatile");
			}
		}
	}
}


#[cfg(feature="volatile")]
pub mod volatile;

#[cfg(not(feature="volatile"))]
pub(crate) mod volatile;

#[cfg(feature="variables")]
pub mod variables;

#[cfg(and(not(feature="variables"),feature="dma"))]
pub(crate) mod variables;

#[cfg(feature="pointer")]
pub mod pointer;

#[cfg(not(feature="pointer"))]
pub(crate) mod pointer;

#[cfg(feature="faults")]
pub mod faults;

#[cfg(feature="dma")]
pub mod dma;

#[cfg(feature="memctl")]
pub mod memctl;

