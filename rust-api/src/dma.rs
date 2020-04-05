#![allow(non_upper_case_globals)]

use crate::pointer::{ShortMutPtr, PackedMutPtr};
use crate::volatile::*;
use crate::LockVBlank;
use core::ops::BitOrAssign;
use core::mem::MaybeUninit;
use crate::variables::LockVBlank;

#[repr(C,packed)]
#[derive(Copy,Clone)]
struct DMAData(u8,u8,PackedMutPtr<()>,u16,MaybeUninit<[u8;9]>);

extern "C"{
    #[no_mangle]
    pub static __dma_data: AtomicCell<[DMAData;8]>;
    #[no_mangle]
    pub static __dma_enable: VolatileCell<u8>;
    #[no_mangle]
    pub static __hdma_enable: VolatileCell<u8>;
}


#[repr(transparent)]
pub struct DMARemoteAddress<T>{

}