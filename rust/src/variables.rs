

use crate::volatile::*;
use core::sync::atomic::{compiler_fence, Ordering};

#[allow(LINK)]
extern "C"{

    #[no_mangle]
    static __counter_enable: VolatileCell<u8>;

    #[no_mangle]
    static __joypads: VolatileCell<[u16;4]>;

    #[no_mangle]
    static __screen_display: VolatileCell<u8>;

    #[no_mangle]
    static _Random: VolatileCell<u16>;
}

pub fn random() -> &VolatileCell<u16>{
    return &unsafe { _Random }
}

pub fn joypads() -> &VolatileCell<[u16;4]> {
    return &unsafe {__joypads};
}

pub fn counter_enable() -> &VolatileCell<u8>{
    return &unsafe{__counter_enable};
}

static mut _DISABLE_VBLANK: u16 = 0;

pub struct LockVBlank;

impl LockVBlank{
    pub fn lock() -> Self{
        counter_enable() &= !0x80u8;
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
                counter_enable() |= 0x80u8
            }
        }
    }
}
