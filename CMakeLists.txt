cmake_minimum_required(VERSION 3.15)
project(snes_dev C)

enable_language(ASM CXX)

set(CMAKE_THREAD_LIBS_INIT "-lpthread")
set(CMAKE_HAVE_THREADS_LIBRARY 1)
set(CMAKE_USE_WIN32_THREADS_INIT 0)
set(CMAKE_USE_PTHREADS_INIT 1)
set(THREADS_PREFER_PTHREAD_FLAG ON)
set(LLVM_ENABLE_PROJECTS "llvm;lld;clang;libc++abi;libunwind")
set(LLVM_EXPERIMENTAL_TARGETS_TO_BUILD "wc65c816")
set(LLVM_TARGET_ARCH "wc65c816")

configure_file(cross/wc65c816.cmake.in cross/wc65c816.cmake @ONLY)

add_subdirectory(llvm-project/llvm)


