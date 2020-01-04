cmake_minimum_required(VERSION 3.15)
project(snes_dev C)

set(CMAKE_C_STANDARD 99)
set(CMAKE_C_COMPILER_TARGET wc65816-snesdev-snes)
set(CMAKE_ASM_COMPILER_TARGET wc65816-snesdev-snes)
set(CMAKE_C_FLAGS "-ffreestanding -nostdlib -nostdinc")
add_subdirectory(qc)
add_subdirectory(libc)

set(CMAKE_C_STANDARD_INCLUDE_DIRECTORIES libc/include)
include_directories(qc/include libc/include include)

add_library(snes_dev STATIC library.c library.h include/snes_dev/Fault.h include/snes_dev/variables.h include/memctl/devices.h)
