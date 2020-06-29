//
// Created by chorm on 2020-02-10.
//

#ifndef SNES_DEV_SETJMP_H
#define SNES_DEV_SETJMP_H

typedef struct __jmp_buf{
    __short_ptr(void) __sp;
    unsigned char __s;
} jmp_buf[1];

int __setjmp(jmp_buf __buf)__attribute__((returns_twice));

void longjmp(jmp_buf __buf,int __ret_val) __attribute__((noreturn));

#define setjmp(__buf) __setjmp(__buf)


#endif //SNES_DEV_SETJMP_H
