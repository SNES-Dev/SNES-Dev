
use core::ops::Deref;
use core::mem::MaybeUninit;

//Pointers are layed out exactly the same way as a PackedPtr, except that there is a trailing padding byte, that can hold any bit-pattern, including uninitialized data

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
    #[inline]
    fn from(val: *const T) -> Self {
        unsafe{
            core::mem::transmute_copy::<Self>(&val)
        }
    }
}

impl<T: ?Sized> From<*mut T> for PackedPtr<T>{
    #[inline]
    fn from(val: *mut T) -> Self {
        unsafe{
            core::mem::transmute_copy::<Self>(&val)
        }
    }
}

impl<T: ?Sized> From<*mut T> for PackedMutPtr<T>{
    #[inline]
    fn from(val: *mut T) -> Self {
        unsafe{
            core::mem::transmute_copy::<Self>(&val)
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
    #[inline]
    fn from(val: PackedPtr<T>) -> Self {
        let mut ret: MaybeUninit<Self> = MaybeUninit::uninit();
        unsafe{
            (ret.as_mut_ptr() as *mut PackedPtr<T>).write(val);
            ret.assume_init()
        }
    }
}


impl<T: ?Sized> From<*T> for ShortPtr<T>{
    #[inline]
    fn from(val: *T) -> Self {
        unsafe{
            core::mem::transmute_copy::<Self>(&val)
        }
    }
}

impl<T: ?Sized> From<*mut T> for ShortPtr<T>{
    #[inline]
    fn from(_: *mut T) -> Self {
        unsafe{
            core::mem::transmute_copy::<Self>(&val)
        }
    }
}

impl<T: ?Sized> From<*mut T> for ShortMutPtr<T>{
    #[inline]
    fn from(_: *mut T) -> Self {
        unsafe{
            core::mem::transmute_copy::<Self>(&val)
        }
    }
}