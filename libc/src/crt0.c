//
// Created by chorm on 2020-01-02.
//

extern char __stack_head[];
extern char __stack_tail[];


typedef void init_fn();

extern init_fn* __init_fn_begin[];
extern init_fn* __init_fn_end[];


extern void main();


static void _start() __attribute__((noreturn,section(".text.init")));


__attribute__((section(".text.init"),naked)) void _Reset()  {
    __asm__ __volatile__("sei");
    __asm__ __volatile__("clc");
    __asm__ __volatile__("xce");
    __asm__ __volatile__("lea __stack_tail"); // Note: This is an alias to lda #imm, but loads the value of a symbol if a symbol is provided
    __asm__ __volatile__("tas");
    __asm__ __volatile__("jmp _start");
    __builtin_unreachable();
}




void _start() __attribute__((noreturn)) {
    for(init_fn** fn = __init_fn_begin;fn!=__init_fn_end;fn++)
        (*fn)();
    main();
    __asm__ volatile("stp");
    __builtin_unreachable();
}
