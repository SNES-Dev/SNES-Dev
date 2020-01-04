//
// Created by chorm on 2020-01-03.
//

#ifndef SNES_DEV_STDDEF_H
#define SNES_DEV_STDDEF_H

#include <qc_int.h>

typedef _UIntShortPtr size_t;
typedef _IntShortPtr ptrdiff_t;

#define NULL 0

#define offsetof(type,member) __builtin_offsetof(type,member)

#if (defined(__STDC__)&&(__STDC_VERSION__ >=201112L))||(defined(__cplusplus)&&(__cplusplus>201103L))
typedef long double max_align_t;
#endif


#endif //SNES_DEV_STDDEF_H
