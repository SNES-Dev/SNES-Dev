//
// Created by chorm on 2020-01-14.
//

#ifndef SNES_DEV_CXX_ABI_H
#define SNES_DEV_CXX_ABI_H

#include <stddef.h>
#include <setjmp.h>

namespace std{
    struct type_info;
}

namespace __cxx_abi{
    typedef void(__copy_ctor)(void*,const void*);
    typedef void(__move_ctor)(void*,void*);
    typedef void(__dtor)(void*);
    typedef void*(__allocator)(size_t);
    typedef void(__deallocator)(void*);

    struct __exception_info{
        const type_info* __rtti;
        __copy_ctor* __copy;
        __dtor* __dtor;
        __allocator* __alloc;
        __deallocator* __dealloc;
        void* __object;
        unsigned short __refc;
    };
    __exception_info* _CxxExceptionAllocate(const void* __except,const type_info* __rtti,__copy_ctor* __copy,__dtor* __dtor,__allocator* __alloc,deallocator* __dtor);
    void _CxxExceptionFree(__exception_info* info);

    void _CxxTerminate() [[noreturn]];
    void _CxxSetTerminate(void(__attribute__((noreturn))*)());

    unsigned short _CxxCreateExceptionLandingPad(jmp_buf __buf,bool(*__check_except)(const __exception_info* inf));

    unsigned short _CxxAddUnwindHandler(void* __frame_base,void(*__unwind)(__exception_info*,void*,void*),bool(*__check_except)(const __exception_info* inf));
    void _CxxUnregister(unsigned short);
}

#endif //SNES_DEV_CXX_ABI_H
