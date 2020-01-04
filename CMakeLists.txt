cmake_minimum_required(VERSION 3.15)
project(snes_dev C)

set(CMAKE_THREAD_LIBS_INIT "-lpthread")
set(CMAKE_HAVE_THREADS_LIBRARY 1)
set(CMAKE_USE_WIN32_THREADS_INIT 0)
set(CMAKE_USE_PTHREADS_INIT 1)
set(THREADS_PREFER_PTHREAD_FLAG ON)

SET(CMAKE_C_STANDARD 99)
add_subdirectory(qc)
add_subdirectory(libc)


set(CMAKE_C_STANDARD_INCLUDE_DIRECTORIES libc/include)
include_directories(qc/include libc/include include)

add_library(snes_dev STATIC library.c library.h include/snes_dev/Fault.h include/snes_dev/variables.h include/memctl/devices.h)
