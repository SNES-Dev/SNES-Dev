cmake_minimum_required(VERSION 3.15)
project(snes_dev C)

set(CMAKE_C_STANDARD 99)
set(CMAKE_C_COMPILER_TARGET wc65816-snesdev-snes)
set(CMAKE_ASM_COMPILER_TARGET wc65816-snesdev-snes)
set(CMAKE_C_FLAGS "-ffreestanding -nostdlib -nostdinc")
add_subdirectory(qc)

include_directories(qc/include)

add_library(snes_dev library.c library.h)