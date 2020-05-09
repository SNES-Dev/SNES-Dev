use core::cell::UnsafeCell;
use core::mem::{MaybeUninit, transmute};
use core::panicking::panic;
use core::ops::{Index, Deref, AddAssign, Add, SubAssign, Sub, Mul, MulAssign, Div, DivAssign, Rem, RemAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};
use crate::InterruptGuard;
use core::sync::atomic::{Ordering, compiler_fence};
use core::marker::PhantomData;
use core::ptr::NonNull;

///
/// Represents a volatile value in memory
/// This type has interior mutability (via UnsafeCell<T>).
/// If T is repr(C), then this is equivalent to the C type volatile T
/// Note that references to the held value can't be obtained due to rust's aliasing requirements
/// And the inherit volatility of
#[repr(transparent)]
pub struct VolatileCell<T: Copy>{
    cell: UnsafeCell<()>,
	phantom: PhantomData<T>
}

unsafe impl<T: Copy + Send> Send for VolatileCell<T>{}

impl<T: Copy> VolatileCell<T>{
	pub unsafe fn wrap(ptr: NonNull<T>)->&Self{
		transmute(ptr)
	}
	pub unsafe fn wrap_nullable(ptr: *mut T)->Option<&Self>{
		transmute(ptr)
	}
    ///
    /// Loads the value stored in the Cell and returns it
    pub fn load(&self) -> T{
        unsafe{core::intrinsics::volatile_load(cell.get() as *const T)}
    }
    pub fn store(&self,val: T){
        unsafe{
            core::intrinsics::volatile_store(cell.get()as *mut T,val)
        }
    }

	pub fn copy_from(&self,other: &VolatileCell<T>){
		if self==other{
			return
		}
		unsafe { core::intrinsics::volatile_copy_nonoverlapping_memory(self.cell.get() as *mut T, other.cell.get() as *mut T, 1) }
	}
}

impl<T: Copy> VolatileCell<MaybeUninit<T>>{
	pub fn store_value(&self,val: T){
		unsafe{core::intrinsics::volatile_store((*self.cell.get()).as_mut_ptr(),val)};
	}
	pub unsafe fn load_value(&self)->T{
		core::intrinsiscs::volatile_load((*self.cell.get()).as_ptr())
	}
	pub unsafe fn unwrap(&self) ->&VolatileCell<T>{
		(*self.cell.get()).as_ptr().cast::<_>() as &_
	}
	pub fn zero(&self){
		unsafe{core::intrinsics::volatile_store(self.cell.get(),MaybeUninit::zeroed())}
	}
}

impl<T: Copy,Size: usize> VolatileCell<[T;Size]>{
	pub fn load_at(&self,idx: isize) -> T{
		if idx >= Size || idx<0{
			panic!("Cannot index array of size {} at {}",Size,idx);
		}
		unsafe{
			core::intrinsics::volatile_load(core::intrinsics::offset(self.cell.get() as *T,idx))
		}
	}
	pub fn store_at(&self,idx: isize,val: T){
		if idx >= Size || idx<0{
			panic!("Cannot index array of size {} at {}",Size,idx);
		}
		unsafe{
			core::intrinsics::volatile_store(core::intrinsics::offset(self.cell.get() as *mut T,idx) as *mut T,val)
		}
	}
}


pub struct LockedVolatileCell<'a,T: Copy>{
	lock: InterruptGuard,
	wrapped: &'a VolatileCell<T>
}

impl<'a,T: Copy> Deref for LockedVolatileCell<'a,T>{
	type Target = VolatileCell<T>;

	fn deref(&self) -> &Self::Target {
		self.wrapped
	}
}

#[repr(transparent)]
pub struct AtomicCell<T: Copy>{
    cell: UnsafeCell<()>,
	phantom: PhantomData<T>
}

unsafe impl<T: Copy + Send> Send for AtomicCell<T>{}
unsafe impl<T: Copy + Send> Sync for AtomicCell<T>{}

impl<T: Copy> AtomicCell<T>{
	pub fn exchange(&self, val: T) -> T{
		let _lock = InterruptGuard::lock();
		unsafe{
			let ret = core::intrinsics::volatile_load(self.cell.get() as *mut T);
			core::intrinsics::volatile_store(self.cell.get() as *mut T,val);
			ret
		}
	}
    pub fn load(&self) -> T{
		let _lock = InterruptGuard::lock();
        unsafe{
            core::intrinsics::volatile_load(self.cell.get() as *mut T)
        }
    }

    pub fn store(&self,val: T) -> (){
		let _lock = InterruptGuard::lock();
        unsafe{
            core::intrinsics::volatile_store(self.cell.get() as *mut T,val);
        }
    }

	pub fn lock(&self) -> LockedVolatileCell<T>{
		LockedVolatileCell{lock: InterruptGuard::lock(),wrapped: unsafe{std::mem::transmute(self)}}
	}

	pub unsafe fn copy_from(&self, other: &AtomicCell<T>){
		if self==other{
			return
		}
		let _lock = InterruptGuard::lock();
		core::intrinsics::volatile_copy_nonoverlapping_memory(self.cell.get(),other.cell.get(),1);
	}
}

impl<T: Copy> AtomicCell<MaybeUninit<T>>{
	pub fn store_value(&self,val: T){
		let _lock = InterruptGuard::lock();
		unsafe{core::intrinsics::volatile_store(self.cell.get() as *mut T,val)};
	}
	pub unsafe fn load_value(&self)->T{
		let _lock = InterruptGuard::lock();
		core::intrinsiscs::volatile_load(self as *const T)
	}
	pub unsafe fn unwrap(&self) ->&AtomicCell<T>{
		transmute(self)
	}
	pub fn zero(&self){
		let _lock = InterruptGuard::lock();
		unsafe{core::intrinsics::volatile_store(self.cell.get() as *mut MaybeUninit<T>,MaybeUninit::zeroed())}
	}
}

#[repr(transparent)]
pub struct VolatileRead<T: Copy>{
	cell: VolatileCell<T>
}

impl<T: Copy> VolatileRead<T>{
	pub fn load(&self) -> T{
		self.cell.load()
	}

	pub fn read_only(v: &VolatileCell<T>) -> &VolatileRead<T>{
		unsafe { std::mem::transmute(v)}
	}
}

#[repr(transparent)]
pub struct AtomicRead<T: Copy>{
	cell: AtomicCell<T>
}

pub struct LockedVolatileRead<'a,T: Copy>{
	lock: InterruptGuard,
	cell: &'a VolatileRead<T>
}

impl<'a,T: Copy> Deref for LockedVolatileRead<'a,T>{
	type Target = VolatileRead<T>;

	fn deref(&self) -> &Self::Target {
		self.cell
	}
}

impl<T: Copy> AtomicRead<T>{
	pub fn load(&self) -> T{
		self.cell.load()
	}

	pub fn read_only(v: &AtomicCell<T>) -> &AtomicRead<T>{
		unsafe { std::mem::transmute(v)}
	}

	pub fn lock(&self) -> LockedVolatileRead<T>{
		LockedVolatileRead{lock: InterruptGuard::lock(),cell: unsafe { std::mem::transmute(self) } }
	}
}

macro_rules! impl_array_deref_for_cell{
	(@ $cellt:ident $($sizes:literal)+) =>{
		$(
			impl<T:Copy> Deref for $cellt<[T;$sizes]>{
				type Output = [$cellt<T>];
				fn deref(&self) -> &Self::Output{
					unsafe{core::slice::from_raw_parts(std::mem::transmute(self),$sizes)}
				}
			}
		)+
	}
	[$cellt:ident] => (impl_array_deref_for_cell!(@ $cellt 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32))
}

impl_array_deref_for_cell![AtomicCell];
impl_array_deref_for_cell![VolatileCell];
impl_array_deref_for_cell![AtomicRead];
impl_array_deref_for_cell![VolatileRead];