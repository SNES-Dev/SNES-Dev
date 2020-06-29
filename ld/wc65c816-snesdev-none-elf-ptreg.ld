SECTIONS {
	.note.snesdev.memctl.flags (NOLOAD){
		. = 4
		SHORT(0x8120)
	}
	.extreg.ptreg ALIGN(16384){
		__ptbase = 0x0100,
		__ptflags = 0x0104
	}
}