//
// Created by chorm on 2020-01-02.
//

#ifndef SNES_DEV_QC_FLOAT_H
#define SNES_DEV_QC_FLOAT_H

#include <qc_base.h>
#include <qc_int.h>

#define __qc_float_impl_bop(type,repr_t,name)\
    __qc_inline __qc_def_bop(type,name){\
        repr_t __a_repr = {.val=a};\
        repr_t __b_repr = {.val=b};\
        return __qc_call_op(repr_t,name,__a_repr,__b_repr).val;\
    }
#define __qc_float_impl_top(type,repr_t,name)\
    __qc_inline __qc_def_top(type,name){\
        repr_t __a_repr = {.val=a};\
        repr_t __b_repr = {.val=b};\
        repr_t __c_repr = {.val=c};\
        return __qc_call_op(repr_t,name,__a_repr,__b_repr,__c_repr).val;\
    }
#define __qc_float_impl_uop(type,repr_t,name)\
    __qc_inline __qc_def_uop(type,name){\
    repr_t __val_repr = {.val=val};\
    return __qc_call_op(repr_t,name,__val_repr).val;\
    }
#define __qc_float_impl_nop(type,repr_t,name)\
    __qc_inline __qc_def_nop(type,name){\
        return __qc_call_op(repr_t,name).val;\
    }

#define __qc_float_bop(type,repr_t,name)\
    __qc_def_bop(repr_t,name);\
    __qc_float_impl_bop(type,repr_t,name)

#define __qc_float_uop(type,repr_t,name)\
    __qc_def_uop(repr_t,name);\
    __qc_float_impl_uop(type,repr_t,name)
#define __qc_float_nop(type,repr_t,name)\
    __qc_def_nop(repr_t,name);\
    __qc_float_impl_nop(type,repr_t,name)
#define __qc_float_top(type,repr_t,name)\
    __qc_def_top(repr_t,name);\
    __qc_float_impl_top(type,repr_t,name)

#define __qc_float_ops(type,repr_t)\
    __qc_float_bop(type,repr_t,add);\
    __qc_float_bop(type,repr_t,sub);\
    __qc_float_bop(type,repr_t,mul);\
    __qc_float_bop(type,repr_t,div);\
    __qc_float_uop(type,repr_t,neg);\
    __qc_float_nop(type,repr_t,inf);\
    __qc_float_nop(type,repr_t,nan);\
    __qc_float_top(type,repr_t,fma);\
    __qc_float_uop(type,repr_t,abs);\
    __qc_float_bop(type,repr_t,max);\
    __qc_float_bop(type,repr_t,min);\
    __qc_float_uop(type,repr_t,exp);\
    __qc_float_uop(type,repr_t,exp2);\
    __qc_float_uop(type,repr_t,expm1);\
    __qc_float_uop(type,repr_t,log);\
    __qc_float_uop(type,repr_t,log10);\
    __qc_float_uop(type,repr_t,log2);\
    __qc_float_uop(type,repr_t,log1p);\
    __qc_float_bop(type,repr_t,pow);\
    __qc_float_uop(type,repr_t,sqrt);\
    __qc_float_uop(type,repr_t,cbrt);\
    __qc_float_bop(type,repr_t,hypot);\
    __qc_float_uop(type,repr_t,sin);\
    __qc_float_uop(type,repr_t,cos);\
    __qc_float_uop(type,repr_t,tan);\
    __qc_float_uop(type,repr_t,asin);\
    __qc_float_uop(type,repr_t,acos);\
    __qc_float_uop(type,repr_t,atan);\
    __qc_float_bop(type,repr_t,atan2);\
    __qc_float_uop(type,repr_t,sinh);\
    __qc_float_uop(type,repr_t,cosh);\
    __qc_float_uop(type,repr_t,tanh);\
    __qc_float_uop(type,repr_t,asinh);\
    __qc_float_uop(type,repr_t,acosh);\
    __qc_float_uop(type,repr_t,atanh);\
    __qc_float_uop(type,repr_t,erf);\
    __qc_float_uop(type,repr_t,erfc);\
    __qc_float_uop(type,repr_t,tgamma);\
    __qc_float_uop(type,repr_t,lgamma)

#define __qc_def_float(type,bits,expbits,sigbits)\
    typedef type _Float##bits __qc_sized_float(bits);\
    typedef union {\
        _Float##bits val;\
        _UInt##bits raw:bits;\
        struct{\
            _UInt##bits sign:1;\
            _UInt##bits exp:expbits;\
            _UInt##bits sig:sigbits;\
        }__repr;\
    }__qc_float_##bits;\
    __qc_float_ops(_Float##bits,__qc_float_##bits)

#define FLT_RADIX 2

__qc_def_float(float,32,7,23);
__qc_def_float(double,64,11,54);
__qc_def_float(long double,128,15,114);
__qc_def_float(float,16,5,12);

#endif //SNES_DEV_QC_FLOAT_H
