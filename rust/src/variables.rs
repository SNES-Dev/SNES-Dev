

use crate::volatile::*;

#[allow(LINK)]
extern "C"{

    #[no_mangle]
    static __counter_enable: VolatileCell<u8>;

    #[no_mangle]
    static __joypads: [VolatileCell<u16>;4];

    #[no_mangle]
    static __screen_display: VolatileCell<u8>;

    #[no_mangle]
    static _Random: VolatileCell<u16>;
}

fn random() -> &VolatileCell<u16>{
    return &unsafe { _Random }
}

fn joypads() -> &[VolatileCell<u16>;4] {
    return &unsafe {__joypads};
}


