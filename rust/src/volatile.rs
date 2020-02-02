use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::panicking::panic;
use core::ops::{Index, Deref, AddAssign, Add, SubAssign, Sub, Mul, MulAssign, Div, DivAssign, Rem, RemAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};
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

	pub fn copy_from(&self,other: &VolatileCell<T>){
		if self==other{
			return
		}
		unsafe { core::intrinsics::volatile_copy_nonoverlapping_memory(self.cell.get(), other.cell.get(), 1) }
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

impl<T: Copy,Size: usize> Index<isize> for VolatileCell<[T;Size]>{
	type Output = VolatileCell<T>;
	fn index(&self,idx: isize) -> &Self::Output{
		if idx >= Size || idx<0{
			panic!("Cannot index array of size {} at {}",Size,idx);
		}
		unsafe{ core::intrinsics::offset(self.cell.get() as *T,idx) as &Self::Output}
	}
}

impl<A,T: Copy+Add<A>> AddAssign<A> for VolatileCell<T>{
	fn add_assign(&self, rhs: A) {
		self.store(self.load()+rhs)
	}
}

impl<A,T: Copy+Sub<A>> SubAssign<A> for VolatileCell<T>{
	fn sub_assign(&self, rhs: A) {
		self.store(self.load()-rhs)
	}
}

impl<A,T: Copy+Mul<A>> MulAssign<A> for VolatileCell<T>{
	fn mul_assign(&self, rhs: A) {
		self.store(self.load()*rhs)
	}
}

impl<A,T: Copy+Div<A>> DivAssign<A> for VolatileCell<T>{
	fn div_assign(&self, rhs: A) {
		self.store(self.load()/rhs)
	}
}

impl<A,T: Copy+Rem<A>> RemAssign<A> for VolatileCell<T>{
	fn rem_assign(&self, rhs: A) {
		self.store(self.load()%rhs)
	}
}

impl<A,T: Copy+BitAnd<A>> BitAndAssign<A> for VolatileCell<T>{
	fn bitand_assign(&self, rhs: A) {
		self.store(self.load()&rhs)
	}
}

impl<A,T: Copy+BitOr<A>> BitOrAssign<A> for VolatileCell<T>{
	fn bitor_assign(&self, rhs: A) {
		self.store(self.load()|rhs)
	}
}

impl<A,T: Copy+BitXor<A>> BitXorAssign<A> for VolatileCell<T>{
	fn bitxor_assign(&self, rhs: A) {
		self.store(self.load()^rhs)
	}
}

pub struct LockedVolatileCell<'a,T: Copy>{
	lock: LockInterrupts,
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

	pub fn lock(&self) -> LockedVolatileCell<T>{
		LockedVolatileCell{lock: LockInterrupts::new(),wrapped: unsafe{std::mem::transmute(self)}}
	}

	pub const unsafe fn zeroed() -> AtomicCell<T>{
		return core::mem::zeroed()
	}

	pub unsafe fn copy_from(&self, other: &AtomicCell<T>){
		if self==other{
			return
		}
		let _ = LockInterrupts::new();
		core::intrinsics::volatile_copy_nonoverlapping_memory(self.cell.get(),other.cell.get(),1);
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
	pub unsafe fn unwrap(&self) ->&AtomicCell<T>{
		(*self.cell.get()).as_ptr().cast::<_>() as &_
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


impl<U,T: Copy+Add<U>> AddAssign<U> for AtomicCell<T>{
	fn add_assign(&self, rhs: U) {
		let lock = self.lock();
		lock.store(lock.load()+rhs);
	}
}

impl<U,T: Copy+Sub<U>> SubAssign<U> for AtomicCell<T>{
	fn sub_assign(&self, rhs: U) {
		let lock = self.lock();
		lock.store(lock.load()-rhs);
	}
}

impl<U,T: Copy+Mul<U>> MulAssign<U> for AtomicCell<T>{
	fn mul_assign(&self, rhs: U) {
		let lock = self.lock();
		lock.store(lock.load()*rhs);
	}
}

impl<U,T: Copy+Div<U>> DivAssign<U> for AtomicCell<T>{
	fn div_assign(&self, rhs: U) {
		let lock = self.lock();
		lock.store(lock.load()/rhs);
	}
}

impl<U,T: Copy+Rem<U>> RemAssign<U> for AtomicCell<T>{
	fn rem_assign(&self, rhs: U) {
		let lock = self.lock();
		lock.store(lock.load()%rhs);
	}
}

impl<U,T: Copy+BitAnd<U>> BitAndAssign<U> for AtomicCell<T>{
	fn bitand_assign(&self, rhs: U) {
		let lock = self.lock();
		lock.store(lock.load()&rhs);
	}
}

impl<U,T: Copy+BitOr<U>> BitOrAssign<U> for AtomicCell<T>{
	fn bitor_assign(&self, rhs: U) {
		let lock = self.lock();
		lock.store(lock.load()|rhs);
	}
}

impl<U,T: Copy+BitXor<U>> BitXorAssign<U> for AtomicCell<T>{
	fn bitxor_assign(&self, rhs: U) {
		let lock = self.lock();
		lock.store(lock.load()^rhs);
	}
}

#[repr(transparent)]
pub struct VolatileRead<T: Copy>{
	cell: UnsafeCell<T>
}

impl<T: Copy> VolatileRead<T>{
	pub fn load(&self) -> T{
		unsafe {core::intrinsics::volatile_load(self.cell.get())}
	}

	pub fn read_only(v: &VolatileCell<T>) -> &VolatileRead<T>{
		unsafe { std::mem::transmute(v)}
	}
}

#[repr(transparent)]
pub struct AtomicRead<T: Copy>{
	cell: UnsafeCell<T>
}

pub struct LockedVolatileRead<'a,T: Copy>{
	lock: LockInterrupts,
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
		let lock = LockInterrupts::new();
		unsafe {core::intrinsics::volatile_load(self.cell.get())}
	}

	pub fn read_only(v: &AtomicCell<T>) -> &AtomicRead<T>{
		unsafe { std::mem::transmute(v)}
	}

	pub fn lock(&self) -> LockedVolatileRead<T>{
		LockedVolatileRead{lock: LockInterrupts::new(),cell: unsafe { std::mem::transmute(self) } }
	}
}