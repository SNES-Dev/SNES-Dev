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



set(SNESDEV_TARGET_VENDOR snesdev CACHE STRING "Target System name for the toolchain")
set(SNESDEV_TARGET_ARCH wc65c816 CACHE STRING "Target Architecture for the toolchain")
set_property(CACHE SNESDEV_TARGET_ARCH PROPERTY STRINGS wc65c816 6502)
set(SNESDEV_TARGET_SYSTEM none CACHE STRING "Target Host name for the toolchain")
set(SNESDEV_TARGET_ABI elf CACHE STRING "Target abi for the toolchain")
set(SNESDEV_TARGET_NAME ${SNESDEV_TARGET_ARCH}-${SNESDEV_TARGET_VENDOR}-${SNESDEV_TARGET_SYSTEM}-${SNESDEV_TARGET_ABI})

set(SNESDEV_TOOLS_DIR ${CMAKE_INSTALL_PREFIX}/${SNESDEV_TARGET_NAME}/tools CACHE PATH "Tools dir to install additional toolchains into")
set(SNESDEV_SYSROOT ${CMAKE_INSTALL_PREFIX}/${SNESDEV_TARGET_NAME}/sysroot CACHE PATH "Sysroot for the installed toolchain")

message(STATUS "Sysroot: ${SNESDEV_SYSROOT}")
message(STATUS "Tool directory: ${SNESDEV_TOOLS_DIR}")
message(STATUS "Targetting: ${SNESDEV_TARGET_NAME}")

configure_file(cross/toolchain.cmake.in cross/toolchain.cmake @ONLY)

add_subdirectory(llvm-project/llvm)


