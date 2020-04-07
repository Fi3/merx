use crate::utils::numeric_methods::*;
use std::convert::TryFrom;

pub trait ArrayWrapper<const LEN: usize>: Sized {
    fn get_array(self) -> [u8; LEN];
    fn from_array_unchecked(value: [u8; LEN]) -> Self;
}

pub trait HasFixedOps<const LEN: usize>: ArrayWrapper<LEN> {
    #[inline(always)]
    fn add_inner(self, rhs: Self) -> Option<Self> {
        match LEN {
            4 => Some(Self::from_array_unchecked(add_buffers_32(
                self.get_array(),
                rhs.get_array(),
            )?)),
            8 => Some(Self::from_array_unchecked(add_buffers_64(
                self.get_array(),
                rhs.get_array(),
            )?)),
            16 => Some(Self::from_array_unchecked(add_buffers_128(
                self.get_array(),
                rhs.get_array(),
            )?)),
            _ => panic!("IsFixed is implemented only fo array of len 4 8 and 16"),
        }
    }

    #[inline]
    fn mul_inner<T: Into<i128>>(self, rhs: T) -> Option<Self> {
        let rhs: i128 = rhs.into();
        match LEN {
            4 => Some(Self::from_array_unchecked(mul_buffer_32(
                self.get_array(),
                rhs as i32,
            )?)),
            8 => Some(Self::from_array_unchecked(mul_buffer_64(
                self.get_array(),
                rhs as i64,
            )?)),
            16 => Some(Self::from_array_unchecked(mul_buffer_128(
                self.get_array(),
                rhs as i128,
            )?)),
            _ => panic!("IsFixed is implemented only fo array of len 4 8 and 16"),
        }
    }

    #[inline]
    fn div_inner<T: Into<i128>>(self, rhs: T) -> Option<Self> {
        let rhs: i128 = rhs.into();
        match LEN {
            4 => Some(Self::from_array_unchecked(div_buffer_32(
                self.get_array(),
                rhs as i32,
            )?)),
            8 => Some(Self::from_array_unchecked(div_buffer_64(
                self.get_array(),
                rhs as i64,
            )?)),
            16 => Some(Self::from_array_unchecked(div_buffer_128(
                self.get_array(),
                rhs as i128,
            )?)),
            _ => panic!("IsFixed is implemented only fo array of len 4 8 and 16"),
        }
    }
}
pub trait HasBound<const MAX: i128, const LEN: usize>: HasFixedOps<LEN> {
    // TODO change names in checked_add ec ecc
    #[inline]
    fn add_checked(self, rhs: Self) -> Option<Self> {
        let sum = self.add_inner(rhs)?.get_array();
        match LEN {
            4 => buffer_is_less_or_equal_32(sum, MAX as i32)?,
            8 => buffer_is_less_or_equal_64(sum, MAX as i64)?,
            16 => buffer_is_less_or_equal_128(sum, MAX as i128)?,
            _ => panic!("HasBound is implemented only fo array of len 4 8 and 16"),
        };
        Some(Self::from_array_unchecked(sum))
    }

    #[inline]
    fn mul_checked(self, rhs: i128) -> Option<Self> {
        let mul = self.mul_inner(rhs)?.get_array();
        match LEN {
            4 => buffer_is_less_or_equal_32(mul, MAX as i32)?,
            8 => buffer_is_less_or_equal_64(mul, MAX as i64)?,
            16 => buffer_is_less_or_equal_128(mul, MAX as i128)?,
            _ => panic!("HasBound is implemented only fo array of len 4 8 and 16"),
        };
        Some(Self::from_array_unchecked(mul))
    }

    #[inline]
    fn div_checked(self, rhs: i128) -> Option<Self> {
        let div = self.div_inner(rhs)?.get_array();
        match LEN {
            4 => buffer_is_less_or_equal_32(div, MAX as i32)?,
            8 => buffer_is_less_or_equal_64(div, MAX as i64)?,
            16 => buffer_is_less_or_equal_128(div, MAX as i128)?,
            _ => panic!("HasBound is implemented only fo array of len 4 8 and 16"),
        };
        Some(Self::from_array_unchecked(div))
    }

    fn from_raw_buffer_checked(buf: [u8; LEN]) -> Option<Self> {
        match LEN {
            4 => buffer_is_less_or_equal_32(buf, MAX as i32)?,
            8 => buffer_is_less_or_equal_64(buf, MAX as i64)?,
            16 => buffer_is_less_or_equal_128(buf, MAX as i128)?,
            _ => panic!("Fixed is implemented only fo array of len 4 8 and 16"),
        };
        Some(Self::from_array_unchecked(buf))
    }
}

pub trait IsFixed<const LEN: usize, const MAX: i128, const POW: u128>:
    Sized + ArrayWrapper<LEN>
{
    fn to_parts(self) -> (i128, i128, u128) {
        match LEN {
            4 => {
                let (int, frac, p) = buffer_to_parts_32::<LEN, POW>(self.get_array());
                (int as i128, frac as i128, p)
            }
            8 => {
                let (int, frac, p) = buffer_to_parts_64::<LEN, POW>(self.get_array());
                (int as i128, frac as i128, p)
            }
            16 => {
                let (int, frac, p) = buffer_to_parts_128::<LEN, POW>(self.get_array());
                (int as i128, frac as i128, p)
            }
            _ => panic!("impossible state"),
        }
    }

    fn is_positive(self) -> bool {
        match LEN {
            4 => buffer_is_positive_32(self.get_array()),
            8 => buffer_is_positive_64(self.get_array()),
            16 => buffer_is_positive_128(self.get_array()),
            _ => panic!("Fixed is implemented only fo array of len 4 8 and 16"),
        }
    }
}

#[macro_export]
macro_rules! get_fixed {
    () => {
        use std::convert::TryFrom;
        use $crate::fixed::{ArrayWrapper, HasBound, HasFixedOps, IsFixed};
        use $crate::utils::numeric_methods::*;

        const I32_LEN: usize = 4;
        const I64_LEN: usize = 8;
        const I128_LEN: usize = 16;

        #[derive(Copy, Clone)]
        #[repr(align(8))]
        pub struct ByteArray<const LEN: usize>([u8; LEN]);

        macro_rules! int_partial_eq {
            ($len:ident, $impl_:ident) => {
                impl PartialEq for ByteArray<$len> {
                    fn eq(&self, rhs: &Self) -> bool {
                        if $impl_(self.0, rhs.0) == std::cmp::Ordering::Equal {
                            true
                        } else {
                            false
                        }
                    }
                }
            };
        }

        int_partial_eq!(I32_LEN, ord_32);
        int_partial_eq!(I64_LEN, ord_64);
        int_partial_eq!(I128_LEN, ord_128);

        macro_rules! int_partial_ord {
            ($len:ident, $impl_:ident) => {
                impl PartialOrd for ByteArray<$len> {
                    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
                        Some($impl_(self.0, rhs.0))
                    }
                }
            };
        }

        int_partial_ord!(I32_LEN, ord_32);
        int_partial_ord!(I64_LEN, ord_64);
        int_partial_ord!(I128_LEN, ord_128);

        impl Eq for ByteArray<I32_LEN> {}
        impl Eq for ByteArray<I64_LEN> {}
        impl Eq for ByteArray<I128_LEN> {}

        macro_rules! int_try_from {
            ($len:ident, $ty_:ty) => {
                impl TryFrom<i128> for ByteArray<$len> {
                    type Error = ();

                    fn try_from(value: i128) -> Result<ByteArray<$len>, Self::Error> {
                        if value <= <$ty_>::max_value() as i128
                            && value >= <$ty_>::min_value() as i128
                        {
                            return Ok(Self::from(value as $ty_));
                        } else {
                            return Err(());
                        }
                    }
                }
            };
        }

        int_try_from!(I32_LEN, i32);
        int_try_from!(I64_LEN, i64);

        macro_rules! int_from {
            ($len:ident, $ty_:ty) => {
                impl From<$ty_> for ByteArray<$len> {
                    fn from(value: $ty_) -> ByteArray<$len> {
                        let inner = <[u8; $len]>::try_from(&value.to_le_bytes()[0..$len]).unwrap();
                        ByteArray(inner)
                    }
                }
            };
        }
        int_from!(I32_LEN, i32);
        int_from!(I64_LEN, i64);
        int_from!(I128_LEN, i128);

        macro_rules! int_into {
            ($len:ident, $ty_:ty) => {
                impl Into<$ty_> for ByteArray<$len> {
                    fn into(self) -> $ty_ {
                        let int = <$ty_>::from_le_bytes(self.0);
                        int
                    }
                }
            };
        }
        int_into!(I32_LEN, i32);
        int_into!(I64_LEN, i64);
        int_into!(I128_LEN, i128);

        impl<const LEN: usize> ArrayWrapper<LEN> for ByteArray<LEN> {
            #[inline(always)]
            fn get_array(self) -> [u8; LEN] {
                self.0
            }
            #[inline]
            fn from_array_unchecked(buf: [u8; LEN]) -> Self {
                ByteArray(buf)
            }
        }

        #[derive(Copy, Clone)]
        #[repr(align(8))]
        pub struct Fixed<const LEN: usize, const MAX: i128, const POW: u128>(ByteArray<LEN>);

        macro_rules! fixed_part_eq {
            ($len:ident) => {
                impl<const MAX: i128, const POW: u128> PartialEq for Fixed<$len, MAX, POW> {
                    fn eq(&self, rhs: &Self) -> bool {
                        self.0 == rhs.0
                    }
                }
            };
        }
        fixed_part_eq!(I32_LEN);
        fixed_part_eq!(I64_LEN);
        fixed_part_eq!(I128_LEN);

        // try_from is implemented just for i128 so is impossible to loose precision when Fixed is constructed
        macro_rules! fixed_try_from {
            ($len:ident, $ty_:ty) => {
                impl<const MAX: i128, const POW: u128> TryFrom<i128> for Fixed<$len, MAX, POW> {
                    type Error = ();

                    fn try_from(value: i128) -> Result<Fixed<$len, MAX, POW>, Self::Error> {
                        debug_assert!(MAX <= <$ty_>::max_value() as i128);
                        let value = value.checked_mul(POW as i128).ok_or(())?;
                        if value.checked_abs().ok_or(())? <= MAX {
                            Ok(Fixed(ByteArray::from(value as $ty_)))
                        } else {
                            Err(())
                        }
                    }
                }
            };
        }
        fixed_try_from!(I32_LEN, i32);
        fixed_try_from!(I64_LEN, i64);
        fixed_try_from!(I128_LEN, i128);

        impl<const LEN: usize, const MAX: i128, const POW: u128> ArrayWrapper<LEN>
            for Fixed<LEN, MAX, POW>
        {
            #[inline]
            fn get_array(self) -> [u8; LEN] {
                let inner = self.0;
                inner.0
            }
            #[inline]
            fn from_array_unchecked(buf: [u8; LEN]) -> Self {
                Fixed(ByteArray(buf))
            }
        }

        impl<const LEN: usize, const MAX: i128, const POW: u128> HasFixedOps<LEN>
            for Fixed<LEN, MAX, POW>
        {
        }

        // Fixed from (decimal_int, decimal_exp) for example if the Fixed has precision = 2
        // (45, 2) -> 0.45    --> 45
        // (45, 1) -> 4.5     --> 450
        // (45, 0) -> 45      --> 4500
        // (405, 1) -> 40.5   --> 4050
        // (405, 2) -> 4.05   --> 405
        // (405, 3) -> None
        macro_rules! fixed_try_from_parts {
            ($len:ident) => {
                impl<const MAX: i128, const POW: u128> TryFrom<(i128, u128)>
                    for Fixed<$len, MAX, POW>
                {
                    type Error = ();

                    fn try_from(value: (i128, u128)) -> Result<Fixed<$len, MAX, POW>, Self::Error> {
                        let precision = value.1;
                        let value = value.0;
                        let self_precision = f64::log10(POW as f64) as u128;
                        if precision > self_precision {
                            return Err(());
                        }
                        let normalized_pow = 10_u128.pow(self_precision as u32 - precision as u32);
                        let normalized_val = value * normalized_pow as i128;
                        if normalized_val <= MAX {
                            let buf =
                                <[u8; $len]>::try_from(&normalized_val.to_le_bytes()[0..$len])
                                    .unwrap();
                            Ok(Self::from_array_unchecked(buf))
                        } else {
                            Err(())
                        }
                    }
                }
            };
        }

        fixed_try_from_parts!(I32_LEN);
        fixed_try_from_parts!(I64_LEN);
        fixed_try_from_parts!(I128_LEN);

        pub enum FloatRounding {
            Up,
            Down,
        }

        macro_rules! fixed_try_from_float {
            ($len:ident) => {
                impl<const MAX: i128, const POW: u128> TryFrom<f64> for Fixed<$len, MAX, POW> {
                    type Error = ();

                    fn try_from(value: f64) -> Result<Fixed<$len, MAX, POW>, Self::Error> {
                        let frac = (POW as f64).log10() as u8;
                        let value = checked_int_from_f64(MAX as u128, frac, value).ok_or(())?;
                        let buf = <[u8; $len]>::try_from(&value.to_le_bytes()[0..$len]).unwrap();
                        Ok(Self::from_array_unchecked(buf))
                        //Self::try_from(value)
                    }
                }
            };
        }

        fixed_try_from_float!(I32_LEN);
        fixed_try_from_float!(I64_LEN);
        fixed_try_from_float!(I128_LEN);

        macro_rules! fixed_try_from_float_rounded {
            ($len:ident) => {
                impl<const MAX: i128, const POW: u128> TryFrom<(f64, FloatRounding)>
                    for Fixed<$len, MAX, POW>
                {
                    type Error = ();

                    fn try_from(
                        value: (f64, FloatRounding),
                    ) -> Result<Fixed<$len, MAX, POW>, Self::Error> {
                        let frac = (POW as f64).log10() as u8;
                        match value.1 {
                            FloatRounding::Up => Self::try_from(
                                round_value_up(MAX as u128, frac, value.0).ok_or(())?,
                            ),
                            FloatRounding::Down => Self::try_from(
                                round_value_down(MAX as u128, frac, value.0).ok_or(())?,
                            ),
                        }
                    }
                }
            };
        }

        fixed_try_from_float_rounded!(I32_LEN);
        fixed_try_from_float_rounded!(I64_LEN);
        fixed_try_from_float_rounded!(I128_LEN);
    };
}
