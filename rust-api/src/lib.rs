#![no_std]
#![no_main]
#![allow(non_upper_case_globals)]
#![feature(asm)]
#![feature(core_intrinsics)]

#![cfg_attr(vendor=lccc,feature(lccc_xlang_attributes))]

use core::sync::atomic::{compiler_fence, Ordering};
use crate::volatile::LockedVolatileCell;
#[cfg(not(target="wc65c816"))]
compile_error!("Can only build snes-dev rust for the 65816 architecture");

static mut _DISABLE_INTERRUPTS: u16 = 0;

pub struct InterruptGuard(());

impl InterruptGuard {
	///
	/// Locks Interrupts and forms a sequentially-consistent barrier with the compiler
	pub fn lock() -> Self{
		unsafe{
			asm!("SEI"::::"volatile");
			compiler_fence(Ordering::SeqCst);
			_DISABLE_INTERRUPTS += 1;
		}
		Self(())
	}

	///
	/// Constructs a Scope Guard that will unlock interrupts on a Drop
	/// The behavior is undefined unless Interrupts are presently disabled
	/// via the issuance of an SEI instruction (or the previous construction of an InterruptGuard)
	pub unsafe fn inherit() -> Self{
		compiler_fence(Ordering::SeqCst);
		_DISABLE_INTERRUPTS += 1;
		Self(())
	}
}

impl Clone for InterruptGuard {
	fn clone(&self) -> Self {
		unsafe{ Self::inherit()}
	}
}

impl Drop for InterruptGuard {
	fn drop(&mut self){
		unsafe {
			_DISABLE_INTERRUPTS-= 1;
			compiler_fence(Ordering::SeqCst);
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

