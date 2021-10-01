#![no_std]
#![no_builtins]

#[cfg(any(target_feature = "float", target_feature = "hardfp", feature = "f16"))]
mod fp16;

#[cfg(any(target_feature = "float", target_feature = "hardfp", feature = "f16"))]
pub use fp16::f16;
