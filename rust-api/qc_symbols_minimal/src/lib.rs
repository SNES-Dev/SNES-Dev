#![no_std]
#![no_core]
#![allow(non_camel_case_types)]

pub unsafe trait __QcType{}

unsafe impl __QcType for u8{}
unsafe impl __QcType for u16{}
unsafe impl __QcType for u32{}
unsafe impl __QcType for u64{}
unsafe impl __QcType for u128{}
unsafe impl __QcType for usize{}
unsafe impl __QcType for i8{}
unsafe impl __QcType for i16{}
unsafe impl __QcType for i32{}
unsafe impl __QcType for i64{}
unsafe impl __QcType for i128{}
unsafe impl __QcType for isize{}
unsafe impl<T> __QcType for *mut T{}
unsafe impl<T> __QcType for *T{}
unsafe impl<T> __QcType for &'_ T{}
unsafe impl<T> __QcType for &'_ mut T{}
unsafe impl __QcType for f32{}
unsafe impl __QcType for f64{}

pub mod __qc_sig_atomic;

pub mod __qc_f32;
pub mod __qc_f64;

