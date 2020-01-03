cmake_minimum_required(VERSION 3.15)
project(qc C)

set(CMAKE_C_STANDARD C99)
set(CMAKE_C_STANDARD 99)
set(CMAKE_C_COMPILER_TARGET wc65816-snesdev-snes)
set(CMAKE_ASM_COMPILER_TARGET wc65816-snesdev-snes)
set(CMAKE_C_FLAGS "-ffreestanding -nostdlib -nostdinc")

include_directories(include)

add_library(qc include/qc_base.h include/qc_int.h src/qc_int.S src/qc_float.c include/qc_float.h)
