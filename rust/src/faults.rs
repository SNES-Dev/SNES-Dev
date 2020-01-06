
use crate::volatile::*;

#[feature(core_intrinsics)]
use core::mem::MaybeUninit;
use core::cell::UnsafeCell;

#[repr(C,u16)]
#[derive(Copy,Clone,Default)]
enum FaultCode{
    Ok = 0,
    Panic = 1,
    WriteViolation = 0x10,
    ReadViolation = 0x11,
    PageTableMalformed = 0x103,
    PageTableIncompatible = 0x104,
    PageTableFault = 0x105,
    DeviceInterrupt0 = 0x1000,
    DeviceInterrupt1 = 0x1001,
    DeviceInterrupt2 = 0x1002,
    DeviceInterrupt3 = 0x1003,
    DeviceInterrupt4 = 0x1004,
    DeviceInterrupt5 = 0x1005,
    DeviceInterrupt6 = 0x1006,
    DeviceInterrupt7 = 0x1007,
    DeviceInterrupt8 = 0x1008,
    DeviceInterrupt9 = 0x1009,
    DeviceInterruptA = 0x100A,
    DeviceInterruptB = 0x100B,
    DeviceInterruptC = 0x100C,
    DeviceInterruptD = 0x100D,
    DeviceInterruptE = 0x100E,
    DeviceInterruptF = 0x100F,
    DeviceInterrupt10 = 0x1010,
    DeviceInterrupt11 = 0x1011,
    DeviceInterrupt12 = 0x1012,
    DeviceInterrupt13 = 0x1013,
    DeviceInterrupt14 = 0x1014,
    DeviceInterrupt15 = 0x1015,
    DeviceInterrupt16 = 0x1016,
    DeviceInterrupt17 = 0x1017,
    DeviceInterrupt18 = 0x1018,
    DeviceInterrupt19 = 0x1019,
    DeviceInterrupt1A = 0x101A,
    DeviceInterrupt1B = 0x101B,
    DeviceInterrupt1C = 0x101C,
    DeviceInterrupt1D = 0x101D,
    DeviceInterrupt1E = 0x101E,
    DeviceInterrupt1F = 0x101F,
    UserDef0 = 0x2000,
    UserDef1 = 0x2001,
    UserDef2 = 0x2002,
    UserDef3 = 0x2003,
    UserDef4 = 0x2004,
    UserDef5 = 0x2005,
    UserDef6 = 0x2006,
    UserDef7 = 0x2007,
    UserDef8 = 0x2008,
    UserDef9 = 0x2009,
    UserDefA = 0x200A,
    UserDefB = 0x200B,
    UserDefC = 0x200C,
    UserDefD = 0x200D,
    UserDefE = 0x200E,
    UserDefF = 0x200F,
    ImplDep0 = 0x2100,
    ImplDep1 = 0x2101,
    ImplDep2 = 0x2102,
    ImplDep3 = 0x2103,
    ImplDep4 = 0x2104,
    ImplDep5 = 0x2105,
    ImplDep6 = 0x2106,
    ImplDep7 = 0x2107,
    ImplDep8 = 0x2108,
    ImplDep9 = 0x2109,
    ImplDepA = 0x210A,
    ImplDepB = 0x210B,
    ImplDepC = 0x210C,
    ImplDepD = 0x210D,
    ImplDepE = 0x210E,
    ImplDepF = 0x210F
}

#[repr(C,u8)]
enum FaultTrigger{
    None = 0,
    Sync = 1,
    Async = 2
}

#[repr(C,packed)]
#[derive(Copy,Clone,Default)]
struct Fault(pub FaultCode,pub MaybeUninit<*mut ()>);

extern "C"{
    #[no_mangle]
    static _Fault: AtomicCell<Fault>;

    #[no_mangle]
    static mut _FaultHandler: Option<*fn (Fault)->()>;
    #[no_mangle]
    static _FaultTriggered: AtomicCell<FaultTrigger>;
}


pub fn set_fault_handler(handler: *fn (Fault)->()){
    unsafe{
        asm!("SEI"::::"volatile");
        _FaultHandler = Some(handler);
        asm!("CLI"::::"volatile");
    }
}

pub unsafe fn raise(f: Fault) -> (){
    loop {
        _Fault.store(f);
        if let FaultTrigger::Async = _FaultTriggered.load(){
            continue;
        }else{
            break;
        }
    }
}

pub fn halt() -> !{
    raise(Fault(FaultCode::Panic,MaybeUninit::uninit()));
    unsafe{
        core::intrinsics::unreachable()
    }
}
