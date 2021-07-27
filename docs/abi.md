# w65 ABI

## Table of Contents


[TOC]



## Types


### Alignment

This ABI mandates an alignment of 2 bytes for all primitive types other than pointers, and 1-byte types. Small (2-byte) pointers also have this 2 byte alignment requirement. Regular Pointers have an alignment of 4 bytes. 1-byte types have an alignment of 1. 


### Type Sizes

The sizes of scalar types are given as follows:



* CHAR_BIT shall be defined as precisely 8. That is, char, unsigned char, and signed char are 8-bit
* unsigned short, short, unsigned int, and int shall all have size 2.
* long, unsigned long, all pointer types, shall have size 4
* long long and unsigned long long shall both have size 8
* All uint_leastN_t and int_leastN_t types shall be the same as the corresponding uintN_t or intN_t type, respectively.
* All uint_fastN_t and int_fastN_t types, except for uint_fast8_t and int_fast8_t, shall be the same as the corresponding uintN_t or uintN_t types.
* The uint_fast8_t and int_fast8_t types shall have size 2
* size_t shall be unsigned int. ptrdiff_t shall be signed int.
* The underlying type of an unscoped enumeration type without a fixed underlying type shall be int. (This applies to repr(C) fieldless enums in rust)


### uint_fast8_t

The uint_fast8_t typedef corresponds to a special type which has size 2, and the following properties:



* It shall be capable of storing and exactly representing any value of type unsigned char
* After storing any value of such a type, passing the value as a parameter, or returning the value from a function, the value of the most significant byte is undefined. The value of the least significant byte is equivalent to the value representation of the value interpreted as type unsigned char.
* When passing a value of type uint_fast8_t to a function, it is passed as a 16-bit value. If all parameters passed in the X or Y registers are of this type or of type int_fast8_t, the value of the x flag is undefined upon entry to the function. 
* When returning a value of type uint_fast8_t, it is returned as a 16-bit value, except that the value of the m flag is undefined upon return to the caller,

The int_fast8_t typedef corresponds to a special type with the same properties as above, except that it shall be capable of storing and exactly representing any value of type signed char.


### Signed Integer Representation

This document defines that signed integers shall be represented using 2s complement. 


### Floating Point Representation

The type float shall be represented as an IEEE 754 binary single precision floating-point value. The type double shall be represented as a binary double precision value. The type `long double` shall be represent as an IEEE 754 binary double precision value.


### Atomics

Atomic operations of size 16 or 8 shall be treated as always lock free. Other sizes of atomics are never treated as lock free. Accessing an non-lock free atomic from an interrupt handler other than BRK or COP has undefined behaviour if the interrupt handler interrupts any non-lock free atomic operation.


## Extended (in-memory) Registers

The SNES-Dev ABI requires that implementations define 8 Extended Registers as symbols, which are available in Bank 0, for storing 4-byte values and additional 8-byte values. These registers shall be stored contiguously in memory, in ascending numerical order, with no padding between each register, and shall be aligned to 4 bytes (and even numbered registers shall be aligned to 8). The registers are indicated by `__r`*`n`* where n is the register number from 0 to 7. 

Given each register, there shall be 3 additional sub-registers, which access part of the memory allocated for the extended register. These registers shall be indicated as `__r`*`n`*`w`, which accesses the lower 2-bytes of `__r`*`n`*, `__r`*`n`*`h`, which access the high byte of  `__r`*`n`*`w`, and `__r`*`n`*`l`, which accesses the lowest order byte of `__r`*`n`*.


## Calling Conventions


### C Calling Convention (Main/Default/Extern C)


#### Registers

The e flag, as well as the S, PC, K registers, and the `__r6`, `__r7`, and `__r8` extended register are all callee saved (non-volatile). All other registers are caller saved (volatile).

Upon entry or exit of a function, the direct page register shall be set to an unspecified value for which all extended registers are reachable from. Upon calling a function, the m flag shall be set. Upon return from a function, the x flag shall be clear. 


#### Parameter and Return Value ABI


##### Small Types



* The first two bool (`_Bool`) parameters are passed inverted in the Z then C flags, all remaining bool (`_Bool`) parameters are treated as 1-byte parameters. bool (`_Bool`) values are returned, inverted, in the Z flag. The return value shall also be present in the A register (the m flag is set on return for bool (`_Bool`) return types). 
    * The value of false shall be 0. The value of true shall be the value 1. The behaviour of returning or passing a bool (`_Bool`) value with any other _object representation_ is undefined.
* The first two 1 or 2 byte parameters are passed (in order) in the X and Y registers. If all of these parameters which exist are 1 byte, then the x flag is set, otherwise, the x flag is clear (including if there are no 1 or 2-byte parameters) and any 1 byte parameters are extended to 2-bytes (zero extended if the source type is unsigned, sign-extended if signed). Remaining parameters use the `__r`*`n`*`w` (for 2 byte parameters) and `__r`*`n`*`l` (for 1 byte parameters) in the order defined for 4 byte parameters, then remaining ones are passed on the stack, 1 or 2 byte parameters are returned in the A register (m is set on return for 1 byte return values, otherwise m is clear, including for return type void, or an larger return type)
    * As a special case, if either such parameter is uint_fast8_t (resp. int_fast8_t) or an equivalent type, the type of the other parameter determines the value of the x flag. If both parameters have this type, then the value of the x flag is indeterminate. If a function returns a value of this type, the value of the m flag is indeterminate. 
* 4 byte parameters (including pointers), and additional 1 or 2 byte parameters, are passed in the extended registers in order `__r1`, `__r2`, `__r3`, `__r4`, `__r5`, `__r6`
* 8 byte (16 byte resp.) parameters are passed as 2 (4 resp.) 4-byte parameters
* 0 sized parameters, and empty classes defined in C++ except for empty classes that are non-trivially for the purposes of abi, are ignored and synthesized as arguments and return values.
* All remaining parameters are pushed to the stack from Right to left. 
* All passed and static normal pointers are treated as 4-byte values. For the purposes of this abi, pointer values are valid and inbounds only if the most significant byte is 0. A program that attempts to dereference a pointer with a most significant byte other than 0 has undefined behaviour. All objects and functions defined by a compliant implementation shall be placed at memory address corresponding to such a pointer.
    * User Code should not rely on this behaviour being preserved in future versions. See [Future Directions](#Future-Directions) for details.
* Varargs parameters are always pushed onto the stack from Right to left. 


##### Other Structs/Unions/Large Values

* Any type not explicitly mentioned above, as well as types defined in C++ that are non-trivial for the purposes of calls, are pushed onto the stack before other parameters, and a pointer to the parameter is passed in the parameter's position. 
* A pointer to the place to store/construct the returned value is passed in `__r0`. The return value is stored/constructed at that pointer, and the pointer is then returned in `__r0`. 

No parameters are passed to any function with an empty parameter list and that returns either void, or does not return.

A regular function which has external linkage, or for which a pointer is created, shall be called with the jsl (Jump to Subroutine Long, opcode $22) instruction and return with the rtl (return from subroutine long, opcode $6B) instruction. 


### Interrupt Calling convention

A special convention is defined to be suitable for use in defining native mode interrupt handlers. Such interrupt handlers must have an empty parameter list and return void. 

The behavior is undefined if an interrupt handler function is called, except by an interrupt vector. Implementations are encouraged to diagnose such violations. 

Interrupt Handlers shall save any register, other than the status register, used by the handler (including any caller saved registers whenever it calls a function). If `__atomic_reset_enabled` is set, the value in X shall be stored to the pointer at `__r1`. `__atomic_reset_enabled` shall then be cleared. 

Returning from a function with this calling convention executes the RTI instruction.

The requirements imposed on interrupt handlers are the same as those imposed on _asynchronous signal handlers_ within the C11 standard, except that the BRK and COP interrupts are treated as _synchronous signal handlers_ (as though triggered by a call to raise/abort). The behaviour is undefined if an interrupt handler exits via an exception or unwinding panic.

The following symbols are defined to correspond with various interrupt vectors used in native mode, and shall be defined with this calling convention (no diagnostic is required):



* __native_cop (Coprocessor Interrupt)
* __native_brk (Software Break)
* __native_irq (Interrupt Request)
* __native_nmi (Non-maskable Interrupt)
* __native_abort (Abort Execution)

The following symbols are defined to correspond with various interrupt vectors used in emulation mode:

* __emulation_cop (Coprocessor Interrupt)
* __emulation_irq (Interrupt Request/Software Break)
* __emulation_nmi (Non-maskable Interrupt)
* __emulation_abort (Abort Execution)
* _Reset (Reset Processor)

Additionally, the following symbols are reserved. The behaviour of a program that defines a symbol with any such name is undefined:
* __emulation_brk
* __emulation_reset
* __native_reset


## Language ABIs


### Itanium C++ ABI

C++ classes that are non-trivial for the purposes of calls use large value abi, regardless of size or alignment. Empty C++ classes that are not non-trivial for the purposes of calls are treated as zero sized types for the purposes of passing and returning values, otherwise are treated as though they have size 1 and alignment 1 (unless modified). 

Pointer to Member Functions do not use the least significant bit to differentiate between vtable offsets and non-virtual function addresses. Rather, it uses the most significant bit. (This permits implementations to pack function definitions together, with no padding between, when space constraints would make requiring even addresses unreasonable). There are no changes to the this-adjustment offset, nor any other changes to the layout of a Pointer-to-member function. 


### Rust

This section applies to all rust implementations that conform to this abi.

The extern"C" and extern"system" abis are equivalent to the C Calling Convention documented above. The extern"platform-intrinsic" abi is reserved for future extensions, but should otherwise be treated as equivalent. Rust implementations shall define the extern"w65-interrupt" abi, to correspond to the Interrupt Calling Convention defined above. All other abis are not specified by this document and excluding the extern"Rust", extern"rust-call", and extern"rust-intrinsic" abi, programs that conform to this ABI shall not use such abis. This ABI imposes no requirements that implementations forbid those abis.


#### LCRust Versioned ABI

This section applies only to rust implementations that implement the LCRust ABI, version 0, or any later version subject to changes in those versions, documented [here](https://hackmd.io/@wSaA8OrrSQ2SlegMvA6e6A/SJ1TeE0y_). 

`extern"Rust"` is equivalent to the C Calling Convention described above. 

The hash algorithm used for computing the hash-part of TypeId shall be FNV-1a 32-bit. 

The correspondence of types, for the purposes of Mangling According to the section following are as follows:
* `u8` and `i8` shall correspond to `unsigned char` and `signed char` respectively
* `u16` and `i16` shall correspond to `unsigned short` and `signed short` respectively
* `u32` and `i32` shall correspond to `unsigned long` and `signed long` respectively.
* `u64` and `i64` shall correspond to `unsigned long long` and `signed long long` respectively.
* `u128` and `i128` shall correspond to the extended `__uint128` and `__int128` types.
* `usize` and `isize` shall be mangled as `u5usize` and `u5isize`.


## SNES-Dev ELF Files

SNES-Dev Binaries are produced intermediately in the ELF format. Other formats such as COFF and PE may be supported in the future. Other formats, such as a.out, are not supported by SNES-Dev. 


### 65816 Common

65816 ELF Files may use the e_machine value EM_65816, 257. 

The class for 65816 elf files is ECLASS32; the order is EDATA2LSB (Little Endian). Use of ECLASS64 or EDATA2MSB is not specified. 

By default, 65816 ELF Files are recommended to use either EOSABINONE (Sys-V) or EOSABISTANDALONE. Use of other abis is not specified.

Use of EOSABINONE is recommended if and only if produced by a compiler (or hand-written assembly) which subscribes to the above defined ABI. 

65816 Relocatable ELF Files may use the following relocations. If a relocation occurs across a bank boundary, the behaviour is undefined. 
* R_WC65816_NONE (0), which performs no relocation
* R_WC65816_ABS24 (1), which performs a 3-byte (long address) relocation against a symbol's absolute long address. Long address relocations are of the form [ll hh bb], where hhll forms the symbol's binary address (value&0xffff), and bb forms the symbol's bank (value&0xff0000>>16). 
* R_WC65816_ABS16 (2), which performs a 2-byte (standard address) relocation against a symbol's bank-local address. Bank-local address relocations are of the form [ll hh], where hhll forms the symbol's binary address. The bank of the symbol is ignored. 
* R_WC65816_REL8 (3), which performs a 1-byte (local) relocation against the lower 1-byte of a symbol's PC Relative address (which is checked relative to the byte immediately before the relocation). If the symbol is not within 256 bytes (wrapping at bank boundaries) of the PC value immediately prior to the relocation, an error shall be issued at link-time
* R_WC65816_REL16 (4), which performs a 2-byte (relative address) relocation against the symbol's pc relative address. If the symbol is not within the same bank, an error shall be issued at link time.
* R_WC65816_BANK (5), which performs a 1-byte (bank) relocation against the symbol's absolute address. Bank relocations are of the form [bb], where bb forms the symbol's bank. 
* R_WC65816_ABS8 (6) which performs a 1-byte relocation against the symbols Direct Page address (the low byte of the symbol's value). It is an error if the symbol's bank is not 0. The behaviour is undefined if the symbol is accessed by this relocation while the symbol is not accessible from the direct page.
* R_WC65816_DIR (7) which performs a 2-byte relocation against the symbols Direct Page Base address (which is the symbol's 16-bit address, rounded down to the last multiple of 256). 
* Relocations 8 and 9 are reserved for future expansion.
* R_WC65816_RELAX_JSL (10), hint that a jsl &lt;long> sequence can be relaxed to jsr &lt;abs> to the relocation symbol if the relocation symbol of this relocation is within the same bank as $. 
* R_WC65816_RELAX_JML (11), hint that a jmp &lt;long> sequence can be replaced with a jmp &lt;abs> sequence, if the destination symbol is within the same bank as $. Also permits replacement with bra rel8 if the destination is within range.
* R_WC65816_RELAX_BRL (12), hint that a brl &lt;rel16> sequence can be replaced by a bra &lt;rel8> sequence, if the destination symbol is within the range of a rel8 jmp.
* R_WC65816_RELAX_DIR (13), hints that an abs16 or abs24 operation can be relaxed to a abs8 operation, if the value is within the 256 bytes following the value of __direct_page, wrapping around bank boundaries.
* R_WC65816_RELAX_ABS (14), hints that an abs24 operation can be relaxed to an abs16 operation, if the value is in the same bank as $. Note: this can only be used correctly if DBR=K. 
* R_WC65816_RELAX_JMP (15), hints that an jmp abs16 can be replaced with a bra rel8 if the destination is in range
* Relocations 16-31 are reserved for future expansion
* Relocations 32-63 may have implementation-defined behaviour.
    * Implementations which use these relocation numbers are recommended to use some mechanism to ensure files produced by other implementations using such numbers expecting different behaviour are not processed. This document does not specify such a mechanism.

When an implementation emits relocations 10-15, it shall also emit valid relocations (as necessary) for the explicitly generated code. Use of relocations 10-15 is advisory only, an implementation need not perform any replacement indicated by any such relocation, and may choose to ignore such relocations instead. 


## Additional Requirements


### Pointer to Function Trampoline

The implementation shall define a function `_FnPtrTrampoline`. The program should not call this function directly. It must be called with JSL. When called with a function pointer in `__r8`, it shall perform a tailcall to that function. Otherwise, the behaviour of the function is undefined. 


### Extensions to this ABI


#### Space Constrained Environments

In a space constrained environment, it may be infeasible to allocate space for Extended registers. In such environments, it is permissible to use only the first 4 extended registers, or none at all. Implementations should document when this is the case, or may be made available. If extended registers are not used, then the Direct Page register is Caller Saved. 


### Future Directions


#### Position Independent Code

The symbol with the name `_GLOBAL_OFFSET_TABLE_` is reserved for future use pertaining to position independent code. The symbol `_DYNAMIC_` is reserved for future use pertaining to position independent code.


#### Additional Extended registers

The symbols of form `__r`, followed by a decimal positive integer are reserved for additional extended registers.


#### Address Space Extensions

The 8 most significant bits of any valid pointer value are reserved for use in extensions that provide an extended address space. User code should not rely on any particular value in future versions, except that no function will have an address with the most significant bit set. 


## Reserved Symbol Glossary

The following is a list of every symbol reserved by this specification. Except where specified, compliant programs may not define these symbols.


### Reserved as Interrupt Handler Functions

These functions are reserved as the name of interrupt handler functions. Complaint Programs may define these symbols according to the requirements prescribed in [Interrupt Calling convention](#Interrupt-Calling-convention):
* __native_cop
* __native_brk
* __native_nmi
* __native_abort
* __native_irq
* __emulation_cop
* __emulation_nmi
* __emulation_abort
* __emulation_irq
* _Reset

Additionally the following symbols are reserved and may not be defined:
* __native_reset
* __emulation_reset
* __emulation_brk


### Reserved For Code Generation

#### Arithmetic Operations

The following symbols are reserved for generating code handling integer, floating-point, and fixed-point operations:
* __fixed_add8
* __uint_add8
* __int_add8
* __fixed_add16
* __uint_add16
* __int_add16
* __fixed_add32
* __uint_add32
* __int_add32
* __fixed_add64
* __uint_add64
* __int_add64
* __fixed_add128
* __uint_add128
* __int_add128
* __fixed_sub8
* __uint_sub8
* __int_sub8
* __fixed_sub16
* __uint_sub16
* __int_sub16
* __fixed_sub32
* __uint_sub32
* __int_sub32
* __fixed_sub64
* __uint_sub64
* __int_sub64
* __fixed_sub128
* __uint_sub128
* __int_sub128
* __fixed_mul8
* __uint_mul8
* __int_mul8
* __fixed_mul16
* __uint_mul16
* __int_mul16
* __fixed_mul32
* __uint_mul32
* __int_mul32
* __fixed_mul64
* __uint_mul64
* __int_mul64
* __fixed_mul128
* __uint_mul128
* __int_mul128
* __fixed_add8
* __uint_add8
* __int_div8
* __fixed_div16
* __uint_div16
* __int_div16
* __fixed_div32
* __uint_div32
* __int_div32
* __fixed_div64
* __uint_div64
* __int_div64
* __fixed_div128
* __uint_div128
* __int_div128
* __fixed_lsh8
* __uint_lsh8
* __int_lsh8
* __fixed_lsh16
* __uint_lsh16
* __int_lsh16
* __fixed_lsh32
* __uint_lsh32
* __int_lsh32
* __fixed_lsh64
* __uint_lsh64
* __int_lsh64
* __fixed_lsh128
* __uint_lsh128
* __int_lsh128
* __fixed_rsh8
* __uint_rsh8
* __int_rsh8
* __fixed_rsh16
* __uint_rsh16
* __int_rsh16
* __fixed_rsh32
* __uint_rsh32
* __int_rsh32
* __fixed_rsh64
* __uint_rsh64
* __int_rsh64
* __fixed_rsh128
* __uint_rsh128
* __int_rsh128
* __float_add16
* __float_add32
* __float_add64
* __float_add128
* __float_sub16
* __float_sub32
* __float_sub64
* __float_sub128
* __float_mul16
* __float_mul32
* __float_mul64
* __float_mul128
* __float_div16
* __float_div32
* __float_div64
* __float_div128
* __float_cmp16
* __float_cmp32
* __float_cmp64
* __float_cmp128
* __float_cvtint8tof16
* __float_cvtint16tof16
* __float_cvtint32tof16
* __float_cvtint64tof16
* __float_cvtint8tof32
* __float_cvtint16tof32
* __float_cvtint32tof32
* __float_cvtint64tof32
* __float_cvtint128tof32
* __float_cvtint8tof64
* __float_cvtint16tof64
* __float_cvtint32tof64
* __float_cvtint64tof64
* __float_cvtint128tof64
* __float_cvtint8tof128
* __float_cvtint16tof128
* __float_cvtint32tof128
* __float_cvtint64tof128
* __float_cvtint128tof128


#### Atomic Operations

The following symbols are reserved for generating code handling atomic operations:
* __atomic_thread_fence
* __atomic_signal_fence
* __atomic_load_n, where _n_ is a decimal number of bytes
* __atomic_store_n, where _n_ is a decimal number of bytes
* __atomic_exchange_n, where _n_ is a decimal number of bytes
* __atomic_compare_exchange_n, where _n_ is a decimal number of bytes
* __atomic_test_and_set
* __atomic_clear


#### Position Independent Code

The following symbols are reserved for future use related to position-independent code: 
* `_GLOBAL_OFFSET_TABLE_`
* `_DYNAMIC_`


#### Misc

The following symbols are reserved for miscellaneous code generation use: 
* `_FnPtrTrampoline`


### Reserved for Register Memory Locations

Any symbol `__r`*`n`*, where n is a decimal number is reserved.


### Reserved for Previous Use

The following symbols were previously used by the specification and are reserved: 

* `_InterruptSavedRegisters`