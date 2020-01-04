#![no_std]
#![no_main]

#![feature(asm)]
#[feature(core_intrinsics)]

use core::ops::Deref;
use core::mem::MaybeUninit;

#[repr(C,packed)]
#[derive(Clone,Copy,PartialEq,PartialOrd,Hash)]
pub struct ShortPtr<T: ?Sized>(u16);

#[repr(C,packed)]
#[derive(Clone,Copy,PartialEq,PartialOrd,Hash)]
pub struct ShortMutPtr<T: ?Sized>(u16);

#[repr(C,packed)]
#[derive(Clone,Copy,Ord, PartialOrd, Eq, PartialEq)]
pub struct PackedPtr<T: ?Sized>(u16,u8);

#[repr(C,packed)]
#[derive(Clone,Copy,Ord, PartialOrd, Eq, PartialEq)]
pub struct PackedMutPtr<T: ?Sized>(u16,u8);

impl<T: ?Sized> From<*T> for PackedPtr<T>{
    fn from(val: *const T) -> Self {
        unsafe{
            core::mem::transmute_copy::<T>(&val)
        }
    }
}

impl<T: ?Sized> From<*mut T> for PackedPtr<T>{
    fn from(val: *mut T) -> Self {
        unsafe{
            core::mem::transmute_copy::<T>(&val)
        }
    }
}

impl<T: ?Sized> From<*mut T> for PackedMutPtr<T>{
    fn from(val: *mut T) -> Self {
        unsafe{
            core::mem::transmute_copy::<T>(&val)
        }
    }
}

impl<T: ?Sized> From<PackedMutPtr<T>> for *mut T{
    fn from(val: PackedMutPtr<T>) -> Self {
        let mut ret: MaybeUninit<Self> = MaybeUninit::uninit();
        unsafe{
            (ret.as_mut_ptr() as *mut PackedMutPtr<T>).write(val);
            ret.assume_init()
        }
    }
}

impl<T: ?Sized> From<PackedMutPtr<T>> for *T{
    fn from(val: PackedMutPtr<T>) -> Self {
        let mut ret: MaybeUninit<Self> = MaybeUninit::uninit();
        unsafe{
            (ret.as_mut_ptr() as *mut PackedMutPtr<T>).write(val);
            ret.assume_init()
        }
    }
}

impl<T: ?Sized> From<PackedPtr<T>> for *T{
    fn from(val: PackedPtr<T>) -> Self {
        let mut ret: MaybeUninit<Self> = MaybeUninit::uninit();
        unsafe{
            (ret.as_mut_ptr() as *mut PackedPtr<T>).write(val);
            ret.assume_init()
        }
    }
}

impl<T: ?Sized> From<*T> for ShortPtr<T>{
    fn from(val: *T) -> Self {
        unsafe{
            core::mem::transmute_copy::<T>(&val)
        }
    }
}

impl<T: ?Sized> From<*mut T> for ShortPtr<T>{
    fn from(_: *mut T) -> Self {
        unsafe{
            core::mem::transmute_copy::<T>(&val)
        }
    }
}

impl<T: ?Sized> From<*mut T> for ShortMutPtr<T>{
    fn from(_: *mut T) -> Self {
        unsafe{
            core::mem::transmute_copy::<T>(&val)
        }
    }
}


pub mod variables;
pub mod volatile;
pub mod faults;
pub mod dma;

pub mod memctl;
