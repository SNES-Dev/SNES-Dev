#![allow(non_upper_case_globals)]

use crate::volatile::*;

extern "C"{
    static __counter_enable: VolatileCell<u8>;
    static __joypad1: VolatileCell<u16>;
    static __joypad2: VolatileCell<u16>;
    static __joypad3: VolatileCell<u16>;
    static __joypad4: VolatileCell<u16>;
    static __screen_display: VolatileCell<u8>;

    static _Random: VolatileCell<u16>;
}



