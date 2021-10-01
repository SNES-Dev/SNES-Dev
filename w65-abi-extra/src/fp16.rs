#[allow(non_camel_case_types)]
#[repr(transparent)]
// TODO: There really should be a repr(float) in rust, but until then, floats and ints have the same abi
#[derive(Copy, Clone, Hash, Default)]
///
/// A type that acts like a 16-bit floating-point type. This has the same layout and ABI as the `_Float16` C type
/// And can be used with the same operations
pub struct f16(u16);

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for f16 {}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for f16 {}

impl f16 {
    pub const ZERO: f16 = f16(0);
    pub const INFINITY: f16 = f16(0x7c00);
    pub const NEG_INFINITY: f16 = f16(0xfc00);
    pub const ONE: f16 = f16(0x4000);
    pub const fn from_bits(x: u16) -> Self {
        Self(x)
    }

    pub const fn to_bits(self) -> u16 {
        self.0
    }

    pub fn from_f32(x: f32) -> Self {
        let bits = x.to_bits();
        let sign = bits >> 31;
        let exp = ((bits & 0x7f800000) >> 23) as i32 - 127;
        let mantissa = bits & 0x7fffff;
        if exp < -5 {
            if exp < -15 {
                return Self(0);
            }
            return Self(((bits >> 13) >> -(exp + 5)) as u16);
        } else if exp == 128 {
            return Self(((mantissa != 0) as u16) | ((sign as u16) << 16) | 0x7c00);
        } else if exp > 5 {
            return Self((sign as u16) << 15 | (0x7c00));
        } else {
            return Self((mantissa >> 13) as u16 | (sign as u16) << 15 | ((exp + 5) as u16) << 10);
        }
    }
}

#[cfg(any(feature = "__ra_test_feature_do_no_use", target_feature = "float"))]
mod fp_impl {
    use core::cmp::Ordering;

    use crate::f16;

    #[repr(i8)]
    pub enum FpCompareResult {
        Less = -1,
        Equal = 0,
        Greater = 1,
        Unordered = 0x7f,
    }
    extern "C" {
        pub fn __add_float16(x: f16, y: f16) -> f16;
        pub fn __sub_float16(x: f16, y: f16) -> f16;
        pub fn __mul_float16(x: f16, y: f16) -> f16;
        pub fn __div_float16(x: f16, y: f16) -> f16;
        pub fn __cmp_float16(x: f16, y: f16) -> FpCompareResult;
    }

    macro_rules! gen_float_impls {
        ($ty:ty $([$trait:ident @ $method:ident, $trait2:ident @ $method2:ident => $fn:ident]),*) => {
            $(
            impl core::ops::$trait for $ty {
                type Output = f16;
                fn $method(self, other: f16) -> f16 {
                    unsafe { $fn(self, other) }
                }
            }

            impl core::ops::$trait2 for $ty {
                fn $method2(&mut self, other: f16) {
                    *self = unsafe { $fn(*self, other) };
                }
            }
            )*
        };
    }

    gen_float_impls!(f16[Add @ add, AddAssign @ add_assign => __add_float16],[Sub @ sub, SubAssign @ sub_assign => __sub_float16],[Mul @ mul, MulAssign @ mul_assign => __mul_float16],[Div @ div, DivAssign @ div_assign => __div_float16]);
    impl PartialEq for f16 {
        fn eq(&self, other: &f16) -> bool {
            return matches!(
                unsafe { __cmp_float16(*self, *other) },
                FpCompareResult::Equal
            );
        }
    }
    impl PartialOrd for f16 {
        fn partial_cmp(&self, other: &f16) -> Option<Ordering> {
            match unsafe { __cmp_float16(*self, *other) } {
                FpCompareResult::Less => Some(Ordering::Less),
                FpCompareResult::Equal => Some(Ordering::Equal),
                FpCompareResult::Greater => Some(Ordering::Greater),
                FpCompareResult::Unordered => None,
            }
        }
    }
}
