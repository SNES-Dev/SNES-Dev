SECTIONS{
	__screen_display = 0x2100;
	__counter_enable = 0x4200;

	__joypad1 = 0x4218;
	__joypad2 = 0x421a;
	__joypad3 = 0x421c;
	__joypad4 = 0x421e;

	__mul_a = 0x4202;
	__mul_b = 0x4203;
	__mul_p = 0x4216;
	__div_d = 0x4204;
	__div_s = 0x4206;
	__div_q = 0x4214;
	__div_r = 0x4216;

	.extreg ALIGN(32768){
		_Fault = 0x00F0;
		_FaultCode = 0x00F0;
		_FaultAddr = 0x00F2;
		_Random = 0x00FE;

		__eax = 0x0000;
		__ax = 0x0002;
		__al = 0x0002;
		__ah = 0x0003;
		__ebx = 0x0004;
		__ax = 0x0006;
		__al = 0x0006;
		__ah = 0x0007;
		__rax = 0x0000;
		__ecx = 0x0008;
		__cx = 0x000a;
		__al = 0x000a;
		__ah = 0x000a;
		__edx = 0x000C;
		__rbx = 0x0008;
		__esi = 0x0010;
		__edi = 0x0014;
		__ebp = 0x0018;
	}
}