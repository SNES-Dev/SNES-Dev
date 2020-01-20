#![no_std]

extern crate snesdev;

use core::panic::PanicInfo;

#[panic_handler]
fn on_panic(info: &PanicInfo) -> !{
    snesdev::faults::halt()
}
