#![allow(non_upper_case_globals)]

#[repr(C,packed)]
struct DMA{
    params: u8,
    ppu_addr: u8,
    addr: *mut (),

}
