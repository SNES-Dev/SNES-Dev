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

#ifndef __SNES_DEV_FLOAT_H
#define __SNES_DEV_FLOAT_H

#define FLT_RADIX 2
#define DECIMAL_DIG 35

#define FLT_DECIMAL_DIG 8
#define DBL_DECIMAL_DIG 16
#define LDBL_DECIMAL_DIG 35

#define FLT_MIN 0x1p-126f
#define DBL_MIN 0x1p-1022
#define LDBL_MIN 0x1p-16382l

#define FLT_MAX 0xFFFFFFp+127f
#define DBL_MAX 0x7FFFFFFFFFFFFFp+1023

#define FLT_EPSILON 0x1p-23f
#define DBL_EPSILON 0x1p-53
#define LDBL_EPSILON 0x1p-113l

#define FLT_DIG 7
#define DBL_DIG 16
#define LDBL_DIG 34

#define FLT_MANT_DIG 24
#define DBL_MANT_DIG 54
#define LDBL_MANT_DIG 114

#define FLT_MIN_EXP -126
#define DBL_MIN_EXP -1022
#define LDBL_MIN_EXP -16382


#define FLT_ROUNDS 2
#define FLT_EVAL_MODE 1

#define FLT_HAS_SUBNORM 1
#define DBL_HAS_SUBNORM 1
#define LDBL_HAS_SUBNORM 1

#endif //__SNES_DEV_FLOAT_H
