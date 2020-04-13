use crate::__QcType;

#[repr(qc_struct(sigatomic_t),align(2))]
pub struct __qc_sigatomic_t(u8);
unsafe impl __QcType for __qc_sigatomic_t{}

extern"rust-intrinsic"{
    //!
    //!  Returns v, exactly, but does not adjust the values of the x or m flags
    //! The behavior is undefined unless it is valid to store T with the current value of the flags
    //! This is only universally permitted for __qc_sigatomic_t
    pub fn __qc_intrinsic_no_storage_size<T: __QcType>(v: T)->T;

    //!
    //! Returns v, bitcasted to U
    //! The behavior is undefined unless either both T and U are the same size,
    //! or U is bitwise narrower than T.
    pub fn __qc_narrow<T: __QcType,U: __QcType>(v: T) -> U;
}


//!
//! Performs a Store Operation for values of type __qc_sigatomic_t
//! This magic intrinsic is inserted (and LTO'd) whenever a sigatomic_t value is stored to.
pub unsafe extern"qc-intrinsic" fn __qc_sigatomic_t_do_store(val: __qc_sigatomic_t) -> __qc_sigatomic_t{
    __qc_intrinsic_no_storage_size(val)
}

pub unsafe extern"qc-intrinsic" fn __qc_sigatomic_t_load_as_u8(val: __qc_sigatomic_t) -> u8{
    __qc_narrow(val)
}

