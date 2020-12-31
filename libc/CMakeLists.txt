cmake_minimum_required(VERSION 3.15)
project(snes_dev_libc C)



set(CMAKE_C_STANDARD 99)
include_directories(include ../qc/include)

add_library(libc STATIC src/crt0.c include/stdint.h src/fault_handler.c include/stdarg.h include/stddef.h include/stdnoreturn.h include/stdalign.h include/limits.h include/float.h include/stdbool.h include/c++/cxx_abi.h src/malloc.c)

if(CMAKE_CXX_COMPILER)
    enable_language(CXX)
    add_library(cxx_abi OBJECT src/cxa_exception.cpp)
    target_link_libraries(libc cxx_abi)
endif()
