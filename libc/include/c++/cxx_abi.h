//
// Created by chorm on 2020-01-14.
//

#ifndef SNES_DEV_CXX_ABI_H
#define SNES_DEV_CXX_ABI_H

#include <stddef.h>

namespace std{
    struct type_info;
}

namespace __cxx_abi{
    struct __exception_info{
        const type_info* __rtti;
        void(*__copy)(void*,const void*);
        void(*__move)(void*,void*);
        void(*__dtor)(void*);
        void*(*__alloc)()
    };
    void* __exception_allocate()
}

#endif //SNES_DEV_CXX_ABI_H
