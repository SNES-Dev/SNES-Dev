//
// Created by chorm on 2020-01-03.
//

#ifndef SNES_DEV_STDARG_H
#define SNES_DEV_STDARG_H

typedef struct{
    void* __arg_base;
    void* __cur_arg;
}__va_list;

typedef __va_list va_list[1];

void __va_start(__va_list* list,void* start,)

#define va_start(list,param)

#endif //SNES_DEV_STDARG_H
