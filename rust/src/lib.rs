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
static mut _DISABLE_VBLANK: u16 = 0;

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

pub struct LockVBlank;

impl LockVBlank{
	pub fn lock() -> Self{
		variables::counter_enable() &= !0x80u8;
		unsafe{
			compiler_fence(Ordering::SeqCst);
			_DISABLE_VBLANK += 1;
		}
		LockVBlank
	}
}

impl Drop for LockVBlank{
	fn drop(&mut self) {
		unsafe{
			_DISABLE_VBLANK -= 1;
			compiler_fence(Ordering::SeqCst);
			if _DISABLE_VBLANK==0{
				variables::counter_enable() |= 0x80u8
			}
		}
	}
}

pub mod variables;
pub mod volatile;
pub mod faults;
pub mod dma;
pub mod pointer;

#[cfg(feature="memctl")]
pub mod memctl;

