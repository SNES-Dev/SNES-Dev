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


INCLUDE ./wc65816-snesdev-vectors.ld
INCLUDE ./wc65816-snesdev-symbols.ld

SECTION {
    .uninit.stack ALIGN(32768){
        . = .;
        __stack_begin = .;
        . = 0x2000;
        __stack_end = .;
    }
	.bss ALIGN(1024){
		*(.bss);
		*(COMMON);
	}
	.uninit ALIGN(65536){
	    . = .
	    . = 0x2000;
	    __heap_head = .;
	    . = 0x1FFFF;
	    __heap_tail = .;
	}

	.note.snesdev.paging.devices (NOLOAD){
		KEEP(*(.note.snesdev.paging.devices))
	}
	.note.snesdev.paging.flags (NOLOAD){
		KEEP(*(.note.snesdev.paging.flags))
	}
}
