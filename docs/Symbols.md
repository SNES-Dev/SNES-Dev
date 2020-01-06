# Linker Provided Symbols

The following symbols are exposed by at link time by the `wc65816-snesdev-symbols.ld` linker script,
 and can be linked against by snesdev programs.

The definitions provide an equivalent C and equivalent rust definition to access directly.
Some symbols overlap, to allow for higher-level abstractions around primitive structures.

## Use of volatile 

All variables described here have volatile semantics,
 many that can be written have side-effects upon writes,
 some have side effects upon reads.

To enforce volatile semantics, the type provided by the snesdev primitive rust library,
`VolatileCell` may be used, and `volatile` may simply be used in C.

`VolatileCell` is defined, as though by:
```rust
#[repr(transparent)]
struct VolatileCell<T: Copy>{
    value: T
}
```

Except that VolatileCell has interior mutability semantics provided by `UnsafeCell<T>`. 

Interacting with a variable encapsulated in a `VolatileCell` is always safe (except from an interrupt handler).

Additionally, some variables also have atomic semantics, and must be written and read with interrupts disabled. 
These variables must be accessed through a variable declared `volatile _Atomic(T)` in C,
 or the AtomicCell primitive.

AtomicCell has the same layout and semantics as VolatileCell, except that accesses are protected by
 disabling interrupts during the accesses.
 
The behavior of accessing any variable declared here,
 except with the correct semantics.
Note that atomic semantics are always correct, even for variables that don't require it. 
(All atomic operations are lock-free, and atomic types have the same layout and representation as the equivalent non-atomic type).

In rust, the position of an array type relative to `VolatileCell` or `AtomicCell` does not matter for access semantic purposes.

## Use of user-provided types 

Programs may use packed structure, union, or enumeration types with the appropriate size
 to represent a variable declared here.
In particular the use of the rust type `MaybeUninit` is permitted for variables which might
 have an indeterminate value. 
In rust, the types must be declared with `#[repr(C)]`, and must be `Copy`.
 Enumeration variants must all be unit variants, and must be exhaustive for the possible values.

Additionally, in C++, the types must be trivially copyable, and have standard layout. 

## Joypad Interaction Registers 

The 4 snes joypad registers are exposed as registers `__joypad1`, `__joypad2`, `__joypad3`, `__joypad4`.
Additionally, all four joypad registers are exposed as an array `__joypads`.

C Declarations:
```c
extern volatile uint16_t __joypad1;
extern volatile uint16_t __joypad2;
extern volatile uint16_t __joypad3;
extern volatile uint16_t __joypad4;
extern volatile uint16_t __joypads[4];
```

Rust Declarations:
```rust
extern "C"{
    #[no_mangle]
    static __joypad1: VolatileCell<u16>;
    #[no_mangle]
    static __joypad2: VolatileCell<u16>;
    #[no_mangle]
    static __joypad3: VolatileCell<u16>;
    #[no_mangle]
    static __joypad4: VolatileCell<u16>;
    #[no_mangle]
    static __joypads: [VolatileCell<u16>;4];
}
```


