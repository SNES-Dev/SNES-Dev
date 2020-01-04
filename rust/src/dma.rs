#![allow(non_upper_case_globals)]

use crate::{ShortMutPtr, PackedMutPtr};
use crate::volatile::*;

#[repr(C,packed)]
#[derive(Copy,Clone)]
struct DMAData(u8,u8,PackedMutPtr<()>,u16,u8,ShortMutPtr<()>,u8);

extern "C"{
    static __dma_data: [VolatileCell<DMAData>;8];
    static __dma_enable: VolatileCell<u8>;
    static __hdma_enable: VolatileCell<u8>;
}

