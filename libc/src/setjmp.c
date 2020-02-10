//
// Created by chorm on 2020-02-10.
//

#include <setjmp.h>

extern void* volatile __ebp;

int __setjmp(jmp_buf __buf){
    __short_ptr(void) __ptr;
    __builtin_wc65c816_acc16();
    __asm__ volatile("TSA");
    __asm__("STA %0":"=r"(__ptr));
    __buf[0].__sp = __ptr;
    __buf[0].__bp = __ebp;
    return 0;
}

extern volatile int __eax;


void longjmp(jmp_buf __buf,int val) __attribute__((noreturn)){
    if(!val)
        val = 1;
    __eax = val;
    __builtin_wc65c816_acc16();
    __ebp = __buf[0].__bp;
    __asm__("LDA %0"::"r"(__buf.__sp))
    __asm__ volatile("TAS");
    __asm__ volatile("RTL");
    __builtin_unreachable();
}
