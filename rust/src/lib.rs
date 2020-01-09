#![no_std]
#![no_main]
#![allow(non_upper_case_globals)]
#![feature(asm)]
#![feature(core_intrinsics)]

#[cfg(not(target="wc65c816"))]
compile_error!("Can only build snes-dev rust for the 65816 architecture");

static mut _DISABLE_INTERRUPTS: u16 = 0;

pub struct LockInterrupts{}

impl LockInterrupts{
	pub fn new() -> LockInterrupts{
		unsafe{
			asm!("SEI"::::"volatile");
			_DISABLE_INTERRUPTS += 1;
		}
	}
}

impl Drop for LockInterrupts{
	fn drop(&mut self){
		_DISABLE_INTERRUPTS -= 1;
		if _DISABLE_INTERRUPTS==0{
			asm!("CLI"::::"volatile");
		}
	}
}


pub(crate) mod variables;
pub(crate) mod volatile;
pub(crate) mod faults;
pub(crate) mod dma;
pub(crate) mod pointer;

pub(crate) mod memctl;

#[cfg(feature="volatile")]
pub use crate::volatile;

#[cfg(feature="variables")]
pub use crate::variables;

#[cfg(feature="faults")]
pub use crate::faults;

#[cfg(feature="dma")]
pub use crate::dma;

#[cfg(feature="pointer")]
pub use crate::pointer;

#[cfg(feature="memctl")]
pub use crate::memctl;