//
// Created by chorm on 2020-02-10.
//

#include <setjmp.h>

int __setjmp(jmp_buf __buf) __attribute__((returns_twice)){
    __short_ptr(void) __ptr;
    unsigned char __s;
    __asm__ volatile("php");
    __asm__ volatile("pla");
    __asm__("sta %0":"=r"(s))
    __builtin_wc65c816_acc16();
    __asm__ volatile("tsa");
    __asm__("sta %0":"=r"(__ptr));
    __buf[0].__sp = __ptr;
    __buf[0].__s = __s;
    return 0;
}

extern volatile int __eax;


void longjmp(jmp_buf __buf,int val) __attribute__((noreturn,noinline)){
    if(!val)
        val = 1;
    __eax = val;
    __asm__("lda %0"::"r"(__buf[0].__s));
    __asm__ volatile("pha");
    __asm__ volatile("plp")
    __builtin_wc65c816_acc16();
    __asm__("lda %0"::"r"(__buf[0].__sp))
    __asm__ volatile("tas");
    __builtin_wc65c816_acc8(); // Satisfy ABI requirements
    __asm__ volatile("rtl"); // Returning from a noreturn function is UB, but I don't care.
    __builtin_unreachable();
}
