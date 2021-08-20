#![no_std]
#![no_builtins]

#[cfg(not(any(doc, target_arch = "w65")))]
core::compile_error!("Cannot compile except for the w65 architecture");
