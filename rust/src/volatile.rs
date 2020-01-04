use core::cell::UnsafeCell;

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
#[repr(transparent)]
struct AtomicCell<T: Copy>{
    cell: UnsafeCell<T>
}

unsafe impl<T: Copy> Send for AtomicCell<T>{}
unsafe impl<T: Copy> Sync for AtomicCell<T>{}

impl<T: Copy> AtomicCell<T>{
    pub fn load(&self) -> T{
        unsafe{
            asm!("SEI"::::"volatile");
            let val = core::intrinsics::volatile_load(self.cell.get());
            asm!("CLI"::::"volatile");
            val
        }
    }

    pub fn store(&self,val: T) -> (){
        unsafe{
            asm!("SEI"::::"volatile");
            core::intrinsics::volatile_store(self.cell.get(),val);
            asm!("CLI"::::"volatile");
        }
    }
}