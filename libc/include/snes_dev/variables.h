//
// Created by chorm on 2020-01-03.
//

#ifndef SNES_DEV_VARIABLES_H
#define SNES_DEV_VARIABLES_H

#include <qc_int.h>

extern volatile _UInt8 __counter_enable;
extern volatile _UInt8 __screen_display;
extern volatile _UInt16 __joypad1;
extern volatile _UInt16 __joypad2;
extern volatile _UInt16 __joypad3;
extern volatile _UInt16 __joypad4;

extern volatile const _UInt16 _Random;

#endif //SNES_DEV_VARIABLES_H
