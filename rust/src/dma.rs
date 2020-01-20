#![allow(non_upper_case_globals)]

use crate::pointer::{ShortMutPtr, PackedMutPtr};
use crate::volatile::*;

#[repr(C,packed)]
#[derive(Copy,Clone)]
struct DMAData(u8,u8,PackedMutPtr<()>,u16,u8,ShortMutPtr<()>,u8);

extern "C"{
    #[no_mangle]
    pub static __dma_data: [VolatileCell<DMAData>;8];
    #[no_mangle]
    pub static __dma_enable: VolatileCell<u8>;
    #[no_mangle]
    pub static __hdma_enable: VolatileCell<u8>;
}

