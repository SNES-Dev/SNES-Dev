// 
// MIT License
// 
// Copyright (c) 2021 SNES-Dev Developers
# 
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
# 
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
# 
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

set(CMAKE_SYSTEM_ARCH w65)

set(SNESDEV_TARGET w65-snesdev-elf)
set(SNESDEV_FORMAT elf32-w65)

find_program(CMAKE_C_COMPILER NAMES $Env{CC} ${SNESDEV_TARGET}-cc ${SNESDEV_TARGET}-gcc ${SNESDEV_TARGET}-clang ${SNESDEV_TARGET}-lccc clang lccc)
find_program(CMAKE_CXX_COMPILER NAME SEnv{CXX} ${SNESDEV_TARGET}-c++ ${SNESDEV_TARGET}-g++ ${SNESDEV_TARGET}-clang ${SNESDEV_TARGET}-lccc clang lccc)
set(CMAKE_ASM_COMPILER ${CMAKE_C_COMPILER})

set(CMAKE_C_COMPILER_TARGET ${SNESDEV_TARGET})
set(CMAKE_CXX_COMPILER_TARGET ${SNESDEV_TARGET})
set(CMAKE_ASM_COMPILER_TARGET ${SNESDEV_TARGET})
