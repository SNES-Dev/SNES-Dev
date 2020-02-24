//
// Created by chorm on 2020-01-02.
//

#ifndef SNES_DEV_QC_BASE_H
#define SNES_DEV_QC_BASE_H

#ifdef __cplusplus
#define __qc_extern extern"C"
#define __qc_inline inline
#else
#define __qc_extern
#define __qc_inline inline static
#endif

#define __qc_def_nop(type,op)\
    __qc_extern type __qc_op_##op##_##type()
#define __qc_def_bop(type,op)\
    __qc_extern type __qc_op_##op##_##type(type a,type b)
#define __qc_def_uop(type,op)\
    __qc_extern type __qc_op_##op##_##type(type val)
#define __qc_def_top(type,op)\
    __qc_extern type __qc_op_##op##_##type(type a,type b,type c)

#define __qc_call_op(type,op,...)\
    __qc_op_##op##_##type(__VA_ARGS__)

#define __QC_SIZE_MODEI_8 byte
#define __QC_SIZE_MODEI_16 HI
#define __QC_SIZE_MODEI_24 PSI
#define __QC_SIZE_MODEI_32 SI
#define __QC_SIZE_MODEI_48 PDI
#define __QC_SIZE_MODEI_64 DI
#define __QC_SIZE_MODEI_128 TI
#define __QC_SIZE_MODEI_256 OI
#define __QC_SIZE_MODEF_16 HF
#define __QC_SIZE_MODEF_32 SF
#define __QC_SIZE_MODEF_64 DF
#define __QC_SIZE_MODEF_128 QF
#define __QC_SIZE_MODEF_long XF

#define __qc_eval(expn) expn
#define __qc_size_int_md(sz) __qc_eval(__QC_SIZE_MODEI_##sz)
#define __qc_size_float_md(sz) __qc_eval(__QC_SIZE_MODEF_##sz)

#define __qc_sized_int(sz) __attribute__((mode(__qc_size_int_md(sz))))
#define __qc_sized_float(sz) __attribute__((mode(__qc_size_float_md(sz))))

#endif //SNES_DEV_QC_BASE_H
