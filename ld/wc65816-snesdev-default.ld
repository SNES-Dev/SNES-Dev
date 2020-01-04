INCLUDE ./wc65816-snesdev-vectors.ld
INCLUDE ./wc65816-snesdev-symbols.ld

SECTION {
	.uninit.stack 0{
		. = .
		__stack_begin = .
		. = 0x2000
		__stack_end = .
	}
	.bss ALIGN(1024){
		*(.bss);
		*(COMMON);
	}

	.note.snesdev.paging.devices (NOLOAD){
		KEEP(*(.note.snesdev.paging.devices))
	}
	.note.snesdev.paging.flags (NOLOAD){
		KEEP(*(.note.snesdev.paging.flags))
	}
}
