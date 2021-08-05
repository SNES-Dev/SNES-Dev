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


SECTIONS {
	section ALIGN(32768) .text.init{
		__start_init = .;
		*(.text.init);
		. = 0x7fc0;
		*(.text.init.desc);
		. = 0x7fe0;
		FILL(0xFFFF);
		SHORT(__native_cop);
		SHORT(__native_brk);
		SHORT(__native_abort);
		SHORT(__native_nmi);
		SHORT(0xFFFF);
		SHORT(__native_irq);
		. = . + 4;
		SHORT(__emulation_cop);
		SHORT(__emulation_abort);
		SHORT(__emulation_nmi);
		SHORT(_Reset);
		SHORT(__emulation_irq);
		__end_init = .;
		PROVIDE(__native_cop = 0xFFFF);
		PROVIDE(__native_brk = 0xFFFF);
		PROVIDE(__native_abort = 0xFFFF);
		PROVIDE(__native_nmi = 0xFFFF);
		PROVIDE(__native_irq = 0xFFFF);
		PROVIDE(__emulation_cop = 0xFFFF);
		PROVIDE(__emulation_abort = 0xFFFF);
		PROVIDE(__emulation_irq = 0xFFFF);
	} = 0xEAEA
}