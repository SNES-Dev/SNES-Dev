cmake_minimum_required(VERSION 3.15)
project(snes_dev_libc C)

set(CMAKE_C_STANDARD 99)
set(CMAKE_C_COMPILER_TARGET wc65816-snesdev-snes)
set(CMAKE_ASM_COMPILER_TARGET wc65816-snesdev-snes)
set(CMAKE_C_FLAGS "-ffreestanding -nostdlib -nostdinc")
include_directories(include ../qc/include)

add_library(libc STATIC src/crt0.c include/stdint.h src/fault_handler.c include/stdarg.h include/stddef.h include/stdnoreturn.h include/stdalign.h include/limits.h include/float.h include/stdbool.h)
