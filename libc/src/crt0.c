//
// Created by chorm on 2020-01-02.
//

extern char __stack_begin[];
extern char __stack_end[];


typedef void init_fn();

extern init_fn* __init_fn_begin[];
extern init_fn* __init_fn_end[];


extern void main();


static void _start() __attribute__((section(".text.init")));

void _Reset() __attribute__((section(".text.init"),naked));

void _Reset() __attribute__((emulation)) {
    __asm__ __volatile__("SEI");
    __asm__ __volatile__("CLC");
    __asm__ __volatile__("XCE");
    __asm__ __volatile__("LEA __stack_begin");
    __asm__ __volatile__("TAS");
    __asm__ __volatile__("JMP _start");
    __builtin_unreachable();
}




void _start() {
    for(init_fn** fn = __init_fn_begin;fn!=__init_fn_end;fn++)
        (*fn)();
    main();
    __asm__ volatile("STP");
    __builtin_unreachable();
}
