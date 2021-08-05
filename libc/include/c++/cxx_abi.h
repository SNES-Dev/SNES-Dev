// 
// MIT License
// 
// Copyright (c) 2021 SNES-Dev Developers
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
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

    void _CxxExceptionThrow(__exception_info* info) __attribute__((noreturn));
    void _CxxTerminate() __attribute__((noreturn));
    void _CxxSetTerminate(void(__attribute__((noreturn))*)());

    unsigned short _CxxCreateExceptionLandingPad(jmp_buf __buf,bool(*__check_except)(const __exception_info* inf));

    unsigned short _CxxAddUnwindHandler(void* __frame_base,void(*__unwind)(__exception_info*,void*,void*),bool(*__check_except)(const __exception_info* inf));
    void _CxxUnregister(unsigned short);
}

#endif //SNES_DEV_CXX_ABI_H
