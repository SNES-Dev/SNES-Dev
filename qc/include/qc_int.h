//
// Created by chorm on 2020-01-02.
//

#ifndef SNES_DEV_QC_INT_H
#define SNES_DEV_QC_INT_H

#include <qc_base.h>

#define __qc_int_ops(type)\
    __qc_def_bop(type,add);\
    __qc_def_bop(type,sub);\
    __qc_def_bop(type,mul);\
    __qc_def_bop(type,div);\
    __qc_def_bop(type,band);\
    __qc_def_bop(type,bor);\
    __qc_def_bop(type,bxor);\
    __qc_def_uop(type,neg);\
    __qc_def_uop(type,bnot);\
    __qc_def_div_ops(type);

#define __qc_def_div_ops(type)\
    typedef struct{ type __quotient; type __remainder;} __div_##type;\
    __qc_extern __div_##type __qc_div_##type(type __divd,type __divs);

#define __qc_def_int_type(type, bits)\
    typedef signed type _Int##bits __qc_sized_int(bits);\
    typedef unsigned type _UInt##bits __qc_sized_int(bits);\
    __qc_int_ops(_Int##bits);\
    __qc_int_ops(_UInt##bits);\

#define __qc_def_ext_int_t(type,salias,ualias,bits)\
    typedef signed type salias __qc_sized_int(bits);\
    typedef unsigned type ualias __qc_sized_int(bits);\
    __qc_int_ops(salias);\
    __qc_int_ops(ualias)

__qc_def_int_type(char,8);
__qc_def_int_type(short,16);
__qc_def_int_type(int,32);
__qc_def_int_type(long long,64);
__qc_def_int_type(long long,128);

__qc_def_ext_int_t(char,_IntSigAtomic,_UIntSigAtomic,16);
__qc_def_ext_int_t(int,_IntPtr,_UIntPtr,24);
__qc_def_ext_int_t(short,_IntShortPtr,_UIntShortPtr,16);


#endif //SNES_DEV_QC_INT_H
