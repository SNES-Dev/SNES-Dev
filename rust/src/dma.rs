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

static DmaScheduled: VolatileCell<u8> = unsafe {core::mem::zeroed()};
static mut DmaIndex: u8 = unsafe { core::mem::zeroed()};

pub fn dma_schedule(a: &'static [u8], dma_addr: u8) -> Result<(),()>{
    let _ = LockVBlank::lock();
    let lock = unsafe{ __dma_data}.lock();
    let idx = unsafe {
        let index = DmaIndex;
        if DmaIndex > 7 { return Err(())}
        DmaIndex += 1;
        index
    };
    DmaScheduled.bitor_assign(1<<idx);
    lock[idx].store(DMAData(0,dma_addr,(a.as_ptr() as *mut u8).into(),a.len() as u16,MaybeUninit::uninit()));
    Ok(())
}

#[no_mangle]
pub unsafe extern"interrupt" fn __native_nmi() -> (){
    if DmaIndex > 0{
        __dma_enable.copy_from(&DmaScheduled);
        DmaScheduled.store(0);
        DmaIndex.store(0);
    }
}