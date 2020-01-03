#![no_std]
#![no_main]

use core::ops::Deref;

#[repr(C,packed)]
#[derive(Clone,Copy,PartialEq,PartialOrd,Hash)]
struct ShortPtr<T: ?Sized>(u16);

#[repr(C,packed)]
#[derive(Clone,Copy,PartialEq,PartialOrd,Hash)]
struct ShortMutPtr<T: ?Sized>(u16);

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


mod variables;
mod volatile;
mod faults;
mod dma;