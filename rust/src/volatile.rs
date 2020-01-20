use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::panicking::panic;
use core::ops::Index;
use crate::LockInterrupts;
use core::sync::atomic::{Ordering, compiler_fence};

///
/// Represents a volatile value in memory
/// This type has interior mutability (via UnsafeCell<T>).
/// If T is repr(C), then this is equivalent to the C type volatile T
/// Note that references to the held value can't be obtained due to rust's aliasing requirements
/// And the inherit volatility of
#[repr(transparent)]
pub struct VolatileCell<T: Copy>{
    cell: UnsafeCell<T>
}

unsafe impl<T: Copy> Sync for VolatileCell<T>{}
unsafe impl<T: Copy> Send for VolatileCell<T>{}

impl<T: Copy> VolatileCell<T>{
    ///
    /// Loads the value stored in the Cell and returns it
    pub fn load(&self) -> T{
        unsafe{core::intrinsics::volatile_load(cell.get())}
    }
    pub fn store(&self,val: T){
        unsafe{
            core::intrinsics::volatile_store(cell.get(),val)
        }
    }
}

impl<T: Copy> VolatileCell<MaybeUninit<T>>{
	pub fn store_value(&self,val: T){
		unsafe{core::intrinsics::volatile_store((*self.cell.get()).as_mut_ptr(),val)};
	}
	pub unsafe fn load_value(&self)->T{
		core::intrinsiscs::volatile_load((*self.cell.get()).as_ptr())
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

impl<T: Copy,Size: usize> Index<isize> for VolatileCell<[T;Size]>{
	type Output = VolatileCell<T>;
	fn index(&self,idx: isize) -> &Self::Output{
		if idx >= Size || idx<0{
			panic!("Cannot index array of size {} at {}",Size,idx);
		}
		unsafe{ core::intrinsics::offset(self.cell.get() as *T,idx) as &Self::Output}
	}
}

#[repr(transparent)]
pub(crate) struct AtomicCell<T: Copy>{
    cell: UnsafeCell<T>
}

unsafe impl<T: Copy> Send for AtomicCell<T>{}
unsafe impl<T: Copy> Sync for AtomicCell<T>{}

impl<T: Copy> AtomicCell<T>{
    pub fn load(&self) -> T{
		let lock = LockInterrupts::new();
        unsafe{
            core::intrinsics::volatile_load(self.cell.get())
        }
    }

    pub fn store(&self,val: T) -> (){
		let lock = LockInterrupts::new();
        unsafe{
            core::intrinsics::volatile_store(self.cell.get(),val);
        }
    }
}

impl<T: Copy> AtomicCell<MaybeUninit<T>>{
	pub fn store_value(&self,val: T){
		let lock = LockInterrupts::new();
		unsafe{core::intrinsics::volatile_store((*self.cell.get()).as_mut_ptr(),val)};
	}
	pub unsafe fn load_value(&self)->T{
		let lock = LockInterrupts::new();
		core::intrinsiscs::volatile_load((*self.cell.get()).as_ptr())
	}
	pub fn zero(&self){
		let lock = LockInterrupts::new();
		unsafe{core::intrinsics::volatile_store(self.cell.get(),MaybeUninit::zeroed())}
	}
}

impl<T: Copy,Size: usize> AtomicCell<[T;Size]>{
	pub fn load_at(&self,idx: isize) -> T{
		if idx >= Size || idx<0{
			panic!("Cannot index array of size {} at {}",Size,idx);
		}
		let lock = LockInterrupts::new();
		unsafe{
			core::intrinsics::volatile_load(core::intrinsics::offset(self.cell.get() as *T,idx))
		}
	}
	pub fn store_at(&self,idx: isize,val: T){
		if idx >= Size || idx<0{
			panic!("Cannot index array of size {} at {}",Size,idx);
		}
		let lock = LockInterrupts::new();
		unsafe{
			core::intrinsics::volatile_store(core::intrinsics::offset(self.cell.get() as *mut T,idx) as *mut T,val)
		}
	}
}

impl<T: Copy,Size: usize> Index<isize> for AtomicCell<[T;Size]>{
	type Output = AtomicCell<T>;
	fn index(&self,idx: isize) -> &AtomicCell<T>{
		if idx >= Size || idx<0{
			panic!("Cannot index array of size {} at {}",Size,idx);
		}
		unsafe{ core::intrinsics::offset(self.cell.get() as *T,idx) as &AtomicCell<T>}
	}
}

