//
// Created by chorm on 2020-01-03.
//

#include <snes_dev/Fault.h>

void __default_fault_handler(Fault fault) __attribute__((weak));

static fault_handler* _FaultHandler = __default_fault_handler;

void set_fault_handler(fault_handler* handler){
    __asm__ volatile("SEI");
    _FaultHandler = handler;
    __asm__ volatile("CLI");
}

__attribute__((convention(interrupt),section(".text.init"))) void __native_irq(void){
    Fault f = _Fault;
    if(_FaultHandler)
        _FaultHandler(f);
    if(f.faultCode==1){
        __asm__ volatile("STP");
        __builtin_unreachable();
    }
}

void raise_fault(Fault f){
    __asm__ volatile("SEI");
    do {
        _Fault.faultAddr = f.faultAddr;
        __asm__ volatile("CLI");
        _Fault.faultCode = f.faultCode;
        __asm__ volatile("SEI");
    }while(_FaultTriggered==FaultASync);
    __asm__ volatile("CLI");

}
