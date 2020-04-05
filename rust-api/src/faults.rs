
use crate::volatile::*;

#[feature(core_intrinsics)]
use core::mem::MaybeUninit;
use core::cell::UnsafeCell;
use crate::InterruptGuard;
use core::sync::atomic::{compiler_fence, Ordering};

#[repr(C,u16,packed)]
#[derive(Copy,Clone)]
pub enum Fault{
    Ok = 0, // Seems odd that I have to represent the absense of a Fault as a Fault itself
    Panic = 1,
    Reserved2 = 2,
    Reserved3 = 3,
    Reserved4 = 4,
    Reserved5 = 5,
    Reserved6 = 6,
    Reserved7 = 7,
    Reserved8 = 8,
    Reserved9 = 9,
    ReservedA = 0xA,
    ReservedB = 0xB,
    ReservedC = 0xC,
    ReservedD = 0xD,
    ReservedE = 0xE,
    ReservedF = 0xF,
    WriteViolation(*()) = 0x10,
    ReadViolation(*()) = 0x11,
    ReservedViolation2(*()) = 0x12,
    ReservedViolation3(*()) = 0x13,
    ReservedViolation4(*()) = 0x14,
    ReservedViolation5(*()) = 0x15,
    ReservedViolation6(*()) = 0x16,
    ReservedViolation7(*()) = 0x17,
    ReservedViolation8(*()) = 0x18,
    ReservedViolation9(*()) = 0x19,
    ReservedViolationA(*()) = 0x1A,
    ReservedViolationB(*()) = 0x1B,
    ReservedViolationC(*()) = 0x1C,
    ReservedViolationD(*()) = 0x1D,
    ReservedViolationE(*()) = 0x1E,
    ReservedViolationF(*()) = 0x1F,
    PageTableMalformed(*()) = 0x100,
    PageTableIncompatible(*()) = 0x101,
    PageTableFault(*()) = 0x102,
    PageTableReserved3(*()) = 0x103,
    PageTableReserved4(*()) = 0x104,
    PageTableReserved5(*()) = 0x105,
    PageTableReserved6(*()) = 0x106,
    PageTableReserved7(*()) = 0x107,
    PageTableReserved8(*()) = 0x108,
    PageTableReserved9(*()) = 0x109,
    PageTableReservedA(*()) = 0x10A,
    PageTableReservedB(*()) = 0x10B,
    PageTableReservedC(*()) = 0x10C,
    PageTableReservedD(*()) = 0x10D,
    PageTableReservedE(*()) = 0x10E,
    PageTableReservedF(*()) = 0x10F,
    DeviceInterrupt0(MaybeUninit<*()>) = 0x1000,
    DeviceInterrupt1(MaybeUninit<*()>) = 0x1001,
    DeviceInterrupt2(MaybeUninit<*()>) = 0x1002,
    DeviceInterrupt3(MaybeUninit<*()>) = 0x1003,
    DeviceInterrupt4(MaybeUninit<*()>) = 0x1004,
    DeviceInterrupt5(MaybeUninit<*()>) = 0x1005,
    DeviceInterrupt6(MaybeUninit<*()>) = 0x1006,
    DeviceInterrupt7(MaybeUninit<*()>) = 0x1007,
    DeviceInterrupt8(MaybeUninit<*()>) = 0x1008,
    DeviceInterrupt9(MaybeUninit<*()>) = 0x1009,
    DeviceInterruptA(MaybeUninit<*()>) = 0x100A,
    DeviceInterruptB(MaybeUninit<*()>) = 0x100B,
    DeviceInterruptC(MaybeUninit<*()>) = 0x100C,
    DeviceInterruptD(MaybeUninit<*()>) = 0x100D,
    DeviceInterruptE(MaybeUninit<*()>) = 0x100E,
    DeviceInterruptF(MaybeUninit<*()>) = 0x100F,
    UserDef0(Option<NonNull<()>>) = 0x2000,
    UserDef1(Option<NonNull<()>>) = 0x2001,
    UserDef2(Option<NonNull<()>>) = 0x2002,
    UserDef3(Option<NonNull<()>>) = 0x2003,
    UserDef4(Option<NonNull<()>>) = 0x2004,
    UserDef5(Option<NonNull<()>>) = 0x2005,
    UserDef6(Option<NonNull<()>>) = 0x2006,
    UserDef7(Option<NonNull<()>>) = 0x2007,
    UserDef8(Option<NonNull<()>>) = 0x2008,
    UserDef9(Option<NonNull<()>>) = 0x2009,
    UserDefA(Option<NonNull<()>>) = 0x200A,
    UserDefB(Option<NonNull<()>>) = 0x200B,
    UserDefC(Option<NonNull<()>>) = 0x200C,
    UserDefD(Option<NonNull<()>>) = 0x200D,
    UserDefE(Option<NonNull<()>>) = 0x200E,
    UserDefF(Option<NonNull<()>>) = 0x200F,
    ImplDep0(MaybeUninit<*()>) = 0x2100,
    ImplDep1(MaybeUninit<*()>) = 0x2101,
    ImplDep2(MaybeUninit<*()>) = 0x2102,
    ImplDep3(MaybeUninit<*()>) = 0x2103,
    ImplDep4(MaybeUninit<*()>) = 0x2104,
    ImplDep5(MaybeUninit<*()>) = 0x2105,
    ImplDep6(MaybeUninit<*()>) = 0x2106,
    ImplDep7(MaybeUninit<*()>) = 0x2107,
    ImplDep8(MaybeUninit<*()>) = 0x2108,
    ImplDep9(MaybeUninit<*()>) = 0x2109,
    ImplDepA(MaybeUninit<*()>) = 0x210A,
    ImplDepB(MaybeUninit<*()>) = 0x210B,
    ImplDepC(MaybeUninit<*()>) = 0x210C,
    ImplDepD(MaybeUninit<*()>) = 0x210D,
    ImplDepE(MaybeUninit<*()>) = 0x210E,
    ImplDepF(MaybeUninit<*()>) = 0x210F
}

#[repr(u8)]
enum FaultTrigger{
    None = 0,
    Sync = 1,
    Async = 2
}

extern "C"{
    #[no_mangle]
    static _Fault: AtomicCell<Fault>;

    #[no_mangle]
    static _FaultHandler: AtomicCell<Option<*extern"C" fn (Fault)->()>>;
    #[no_mangle]
    static _FaultTriggered: AtomicCell<FaultTrigger>;
}


pub fn fault_handler() -> &AtomicCell<Option<*extern"C" fn(Fault)->()>>{
    unsafe{&_FaultHandler}
}

pub fn raise(f: Fault) -> (){
    if let Fault::Ok = f{
        return;
    }
    loop {
        unsafe { _Fault .store(f)}
        asm!("WAI"::::"volatile");
        if let FaultTrigger::Async = unsafe { _FaultTriggered.load()}{
            continue;
        }else{
            compiler_fence(Ordering::SeqCst);
            break;
        }
    }
}

pub fn halt() -> !{
    unsafe{
		raise(Fault::Panic);
        core::intrinsics::unreachable()
    }
}
