cmake_minimum_required(VERSION 3.15)
project(snes_dev C)

enable_language(ASM CXX)

set(CMAKE_THREAD_LIBS_INIT "-lpthread")
set(CMAKE_HAVE_THREADS_LIBRARY 1)
set(CMAKE_USE_WIN32_THREADS_INIT 0)
set(CMAKE_USE_PTHREADS_INIT 1)
set(THREADS_PREFER_PTHREAD_FLAG ON)
set(LLVM_ENABLE_PROJECTS "llvm;lld;clang;libcxxabi;libunwind")
set(LLVM_EXPERIMENTAL_TARGETS_TO_BUILD "wc65c816")

SET(CMAKE_C_STANDARD 99)
add_subdirectory(llvm-project/llvm)


