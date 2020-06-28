//
// Created by chorm on 2020-01-03.
//

#ifndef SNES_DEV_STDARG_H
#define SNES_DEV_STDARG_H


typedef __builtin_va_list va_list;


#define va_start(list,param) __builtin_va_start(list,param)
#define va_copy(list,to) __builtin_va_copy(list,to)
#define va_end(list) __builtin_va_end(list)
#define va_arg(list,T) __builtin_va_arg(list,T)

#endif //SNES_DEV_STDARG_H
