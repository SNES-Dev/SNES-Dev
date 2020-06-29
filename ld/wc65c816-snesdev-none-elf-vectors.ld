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