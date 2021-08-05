// 
// MIT License
// 
// Copyright (c) 2021 SNES-Dev Developers
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#ifndef SNES_DEV_STDINT_H
#define SNES_DEV_STDINT_H

#include <qc_int.h>

typedef _UInt8 uint8_t;
typedef _UInt16 uint16_t;
typedef _UInt32 uint32_t;
typedef _UInt64 uint64_t;
typedef _UInt128 uint128_t;
typedef _UInt128 uintmax_t;

typedef _Int8 int8_t;
typedef _Int16 int16_t;
typedef _Int32 int32_t;
typedef _Int64 int64_t;
typedef _Int128 int128_t;
typedef _Int128 intmax_t;

typedef _UInt8 uint_least8_t;
typedef _UInt16 uint_least16_t;
typedef _UInt32 uint_least32_t;
typedef _UInt64 uint_least64_t;

typedef _Int8 int_least8_t;
typedef _Int16 int_least16_t;
typedef _Int32 int_least32_t;
typedef _Int64 int_least64_t;

typedef _UIntSigAtomic uint_fast8_t;
typedef _IntSigAtomic  int_fast8_t;
typedef _UInt16 uint_fast16_t;
typedef _Int16 int_fast16_t;
typedef _UInt32 uint_fast32_t;
typedef _Int32 int_fast32_t;
typedef _UInt64 uint_fast64_t;
typedef _Int64 int_fast64_t;

typedef _IntPtr intptr_t;
typedef _UIntPtr uintptr_t;
typedef _IntShortPtr  int_shortptr_t;
typedef _UIntShortPtr  uint_shortptr_t;

#define INT8_C(val) val##i8
#define INT16_C(val) val##i16
#define INT32_C(val) val##i32
#define INT64_C(val) val##i64
#define INT128_C(val) val##i128
#define INTMAX_C(val) INT128_C(val)

#define INT8_MIN INT8_C(-128)
#define INT16_MIN INT16_C(-32768)
#define INT32_MIN INT32_C(-2147483648)
#define INT64_MIN INT64_C(-9223372036854775808)
#define INT128_MIN INT128_C(-170141183460469231731687303715884105728)
#define INTMAX_MIN INT128_MIN

#define INT8_MAX INT8_C(127)
#define INT16_MAX INT16_C(32767)
#define INT32_MAX INT32_C(2147483647)
#define INT64_MAX INT64_C(9223372036854775807)
#define INT128_MAX INT128_C(170141183460469231731687303715884105727)
#define INTMAX_MAX INT128_MAX

#define UINT8_MIN UINT8_C(0)
#define UINT16_MIN UINT16_C(0)
#define UINT32_MIN UINT32_C(0)
#define UINT64_MIN UINT64_C(0)
#define UINT128_MIN UINT128_C(0)
#define UINTMAX_MIN UINT128_MIN

#define UINT8_MAX UINT8_C(255)
#define UINT16_MAX UINT16_C(65535)
#define UINT32_MAX UINT32_C(4294967295)
#define UINT64_MAX UINT64_C(18446744073709551615)
#define UINT128_MAX UINT128_C(340282366920938463463374607431768211455)

#endif //SNES_DEV_STDINT_H
