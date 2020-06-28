//
// Created by chorm on 2020-02-10.
//

#include <setjmp.h>

int __setjmp(jmp_buf __buf){
    __short_ptr(void) __ptr;
    __builtin_wc65c816_acc16();
    __asm__ volatile("tsa");
    __asm__("sta %0":"=r"(__ptr));
    __buf[0].__sp = __ptr;
    return 0;
}

extern volatile int __eax;


void longjmp(jmp_buf __buf,int val) __attribute__((noreturn)){
    if(!val)
        val = 1;
    __eax = val;
    __builtin_wc65c816_acc16();
    __asm__("lda %0"::"r"(__buf.__sp))
    __asm__ volatile("tas");
    __asm__ volatile("rtl"); // Returning from a noreturn function is UB, but I don't care.
    __builtin_unreachable();
}
