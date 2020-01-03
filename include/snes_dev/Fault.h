//
// Created by chorm on 2020-01-03.
//

#ifndef SNES_DEV_FAULT_H
#define SNES_DEV_FAULT_H

#include <qc_int.h>

typedef struct Fault{
    _UInt16 faultCode;
    void* faultAddr;
}__attribute__((packed)) Fault;

enum FaultKind{
 FaultNone = 0,
 FaultSync = 1,
 FaultASync = 2
};

extern volatile Fault _Fault;
extern volatile _UInt16 _FaultCode;
extern void* volatile _FaultAddress;
extern volatile _UInt8 _FaultTriggered;

typedef void fault_handler(Fault);

void set_fault_handler(fault_handler*);

void raise_fault(Fault);
void halt();

#endif //SNES_DEV_FAULT_H
