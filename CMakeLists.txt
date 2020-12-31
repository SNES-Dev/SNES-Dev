cmake_minimum_required(VERSION 3.15)
project(snes_dev C)

enable_language(ASM CXX)
set(CMAKE_MODULE_PATH ${CMAKE_CURRENT_SOURCE_DIR}/cmake ${CMAKE_MODULE_PATH})

set(SNESDEV_TARGET_VENDOR snesdev CACHE STRING "Target System name for the toolchain")
set(SNESDEV_TARGET_ARCH wc65c816 CACHE STRING "Target Architecture for the toolchain")
set_property(CACHE SNESDEV_TARGET_ARCH PROPERTY STRINGS wc65c816 6502)
set(SNESDEV_TARGET_SYSTEM none CACHE STRING "Target Host name for the toolchain")
set(SNESDEV_TARGET_ABI elf CACHE STRING "Target abi for the toolchain")
set(SNESDEV_TARGET_NAME ${SNESDEV_TARGET_ARCH}-${SNESDEV_TARGET_VENDOR}-${SNESDEV_TARGET_SYSTEM}-${SNESDEV_TARGET_ABI})

set(SNESDEV_SYSROOT ${CMAKE_INSTALL_PREFIX}/${SNESDEV_TARGET_NAME}/sysroot CACHE PATH "Sysroot for the installed toolchain")

message(STATUS "Sysroot: ${SNESDEV_SYSROOT}")
message(STATUS "Tool directory: ${SNESDEV_TOOLS_DIR}")
message(STATUS "Targetting: ${SNESDEV_TARGET_NAME}")

configure_file(cross/toolchain.cmake.in cross/toolchain.cmake @ONLY)


set(SNESDEV_TOOLCHAIN_MODE internal CACHE STRING "Sets the mode building the snesdev toolchain. If the toolchain is provided by the host system (IE. llvm built with wc65c816 support), set this to host. If SNES-Dev is embedded in a larger project that provides an assembler, C compiler, and linker, set this to external, and also set SNESDEV_LOCAL_C_COMPILER and SNESDEV_LOCAL_LINKER (optionally SNESDEV_LOCAL_CXX_COMPILER).
 If set to host, optionally provide a valid snesdev toolchain file in SNESDEV_HOST_TOOLCHAIN (otherwise the toolchain file built by SNESDEV is used.")
set_property(CACHE SNESDEV_TOOLCHAIN_MODE PROPERTY STRINGS internal external host)

message(STATUS "Using Toolchain ${SNESDEV_TOOLCHAIN_MODE}")

if(SNESDEV_TOOLCHAIN_MODE STREQUAL internal)
    set(LLVM_EXPERIMENTAL_TARGETS_TO_BUILD "wc65c816")
    list(APPEND LLVM_ENABLE_PROJECTS clang lld)
    add_subdirectory(llvm-project/llvm)
    set(SNESDEV_LOCAL_SYSROOT ${CMAKE_CURRENT_BINARY_DIR}/tmp-sysroot)
    set(SNESDEV_LOCAL_LD $<TARGET_FILE:lld>)
    set(SNESDEV_FORCE_GNUDRIVER -flavor gnu)
    set(SNESDEV_LOCAL_C_COMPILER $<TARGET_FILE:clang>)
    set(SNESDEV_LOCAL_CXX_COMPILER $<TARGET_FILE:clang>)
    set(_SNESDEV_LOCAL_PROJECT_DEPENDENCIES clang lld)
    set(_SNESDEV_USE_LOCAL_TOOLCHAIN ON)
elseif(SNESDEV_TOOLCHAIN_MODE STREQUAL external)
    if(NOT DEFINED SNESDEV_LOCAL_C_COMPILER OR NOT DEFINED SNESDEV_LOCAL_LD)
        message(ERROR "External Toolchains must set SNESDEV_LOCAL_C_COMPILER and SNESDEV_LOCAL_LD")
    endif()
    if(TARGET ${SNESDEV_LOCAL_C_COMPILER})
        list(APPEND _SNESDEV_LOCAL_PROJECT_DEPENDENCIES ${SNESDEV_LOCAL_C_COMPILER})
        set(SNESDEV_LOCAL_C_COMPILER $<TARGET_FILE:${SNESDEV_LOCAL_C_COMPILER}>)
    endif()
    if(TARGET ${SNESDEV_LOCAL_LD})
        list(APPEND _SNESDEV_LOCAL_PROJECT_DEPENDENCIES ${SNESDEV_LOCAL_LD})
        set(SNESDEV_LOCAL_C_COMPILER $<TARGET_FILE:${SNESDEV_LOCAL_LD}>)
    endif()
    if(NOT DEFINED SNESDEV_LOCAL_CXX_COMPILER)
        # C++ is not required, but if its available, certain abi stuff is built there
        set(SNESDEV_LOCAL_CXX_COMPILER OFF)
    endif()
    set(_SNESDEV_USE_LOCAL_TOOLCHAIN ON)
elseif(SNESDEV_TOOLCHAIN_MODE STREQ host)
    set(SNESDEV_TOOLCHAIN_FILE ${CMAKE_CURRENT_BINARY_DIR}/cross/toolchain.cmake CACHE FILEPATH "Path to the Toolchain file to use for building local projects")
endif()

if(_SNESDEV_USE_LOCAL_TOOLCHAIN)
    configure_file(cross/local_toolchain.cmake.in cross/local_toolchain.cmake @ONLY)
    set(SNESDEV_TOOLCHAIN_FILE ${CMAKE_CURRENT_BINARY_DIR}/cross/local_toolchain.cmake)
    set(_SNESDEV_LOCAL_CMAKE_ARGS -DSNESDEV_LOCAL_C_COMPILER=${SNESDEV_LOCAL_C_COMPILER}
            -DSNESDEV_LOCAL_LD=${SNESDEV_LOCAL_LD}
            -DSNESDEV_LOCAL_CXX_COMPILER=${SNESDEV_LOCAL_CXX_COMPILER}
            -DSNESDEV_FORCE_GNUDRIVER=${SNESDEV_FORCE_GNUDRIVER})
endif()

include(toolchain_project)


add_toolchain_project(qc)
add_toolchain_project(libc)

