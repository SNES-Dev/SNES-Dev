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


#define va_start(list,param) (list[0].__arg_base = &param+1,list[0].__cur_arg = &param+1)
#define va_copy(list,to) (list[0] = to[0])
#define va_end(list)
#define va_arg(list,T) *((T*)list[0].__arg_base)++

#endif //SNES_DEV_STDARG_H
