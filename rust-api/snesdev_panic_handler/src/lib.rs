#![feature(panic_info_message)]
#![no_std]

use core::panic::PanicInfo;
use core::ptr::NonNull;

#[repr(C)]
pub struct __jmp_buf{
    stack_ptr: *(),

}

#[repr(C)]
pub struct type_info;

#[repr(C)]
pub struct ExceptionInfo{
    except: *(),
    copy_ctor: Option<*fn(*mut (),*())>,
    dtor: Option<*fn(*mut ())>,
    size: usize,
    rtti: *()
}

extern"C"{
    #[no_mangle]
    pub fn _CxxAllocateException(input: *(),copy: Option<*unsafe extern"C" fn(*mut (),*())>,dtor: Option<*unsafe extern"C" fn(*mut ())>,asize: usize,rtti: &'static type_info) -> NonNull<ExceptionInfo>;
    pub fn _CxxBeginExcept(e: NonNull<ExceptionInfo>) -> !;

    pub fn from_rust_rtti(t: TypeInfo) -> &'static type_info;
}

unsafe extern"C" fn get_copy<T: Clone>(t: &T) -> Option<*unsafe extern"C" fn(*mut (),*())>{
    Some(&copy_ctor::<T>)
}

unsafe extern"C" fn copy_ctor<T: Clone>(a: *mut (),b:* ()){
    let val = (b.cast::<T>()).clone();
    a.cast::<T>().write(val)
}

unsafe extern"C" fn dtor<T>(a: *mut ()){
    core::ptr::drop_in_place(a.cast::<T>())
}

macro_rules! throw{
    ($a:stmt) => {
        unsafe{
            let val = $a;
            let rtti = from_rust_rtti(val.type_id());
            _CxxAllocateException(core::mem::transmute(&val),Some(&copy_ctor))
        }
    }
}

#[panic_handler]
fn on_panic(info: &PanicInfo) -> !{
    if let Some(&args) = info.message(){

    }
}
