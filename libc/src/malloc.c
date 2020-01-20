//
// Created by chorm on 2020-01-17.
//

#include <stdint.h>

struct __head_descriptor{
    uint128_t __alloc_flgs;
    uint8_t __heap_length;
};

struct __paging_block{
    uint8_t __continuous_size;
    uint8_t __page_root;
};

extern char __heap_head[];
extern char __heap_tail[];

static __head_descriptor* __heap_descriptor;
static void* __heap_init_pos;

static void __init_heap(void){
    if(!__heap_descriptor){
        __heap_descriptor = (struct __head_descriptor*)__heap_head;
        __heap_descriptor->__heap_length = (__heap_tail-__heap_head)/1024 -1;
        __heap_descriptor->__alloc_flgs = 0;
        __heap_init_pos = __heap_head + 1024;
    }
}

__attribute__((alloc_size(0))) void* malloc(size_t __blck_sz){
    if(!__heap_descriptor)
        __init_heap();
    size_t blocks = __blck_sz/1024 + !!(__blck_sz%1024);
    if (blocks>__heap_descriptor->__heap_length/2)
        return 0;
    ::atomic
}
