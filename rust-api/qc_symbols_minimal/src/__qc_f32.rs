#![allow(non_camel_case_types)]

use crate::__QcType;

#[repr(qc_struct(float32))]
pub union __qc_float32{
    fp: f32,
    bits: u32
}

unsafe impl __QcType for __qc_float32{}
