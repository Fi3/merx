use std::ops;
use std::convert::TryFrom;
use std::convert::TryInto;
use crate::fixed::IsFixed;

pub const fn get_inner_len(max_value: i128) -> usize {
    match max_value as u128 {
        0..=2147483647 => 4,
        2147483648..=9223372036854775807 => 8,
        9223372036854775808..=170141183460469231731687303715884105727 => 16,
        _ => panic!("Too big"),
    }
}

pub const fn pow_10(exp: u8) -> u128 {
    let mut x: u128 = 10;
    let mut i: u8 = 0;
    if exp == 0 {
        return 1 as u128
    } else if exp == 1 {
        return 10 as u128
    }
    while i < exp - 1 {
        x = x * 10;
        i = i + 1;
    }
    x
}

pub trait CheckedOps: Sized {
    fn add_checked(self, rhs: Self) -> Option<Self>;
    fn mul_checked(self, rhs: i128) -> Option<Self>;
    fn div_checked(self, rhs: i128) -> Option<Self>;
    fn is_positive(&self) -> bool;
}

// TODO Display
#[derive(Debug, Hash, Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
#[repr(align(8))]
/// Wrap a numeric value smaller than 0.
///
/// `Debt(-4) + Debt(-6) = Debt(-10)`
///
/// `Debt(-4) + Credit(6) = Credit(2)`
pub struct Debt<T: CheckedOps>(T);

#[derive(Debug, Hash, Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
#[repr(align(8))]
/// Wrap a numeric value bigger or equal than 0.
///
/// `Credit(4) + Credit(6) = Credit(10)`
///
/// `Credit(6) - Debt(6) = Credit(0)`
pub struct Credit<T: CheckedOps>(T);

// Credit + Credit -> Credit?
impl<T: CheckedOps> ops::Add<Credit<T>> for Credit<T> {
    type Output = Option<Credit<T>>;

    #[inline]
    fn add(self, _rhs: Credit<T>) -> Self::Output {
        Some(Credit(self.0.add_checked(_rhs.0)?))
    }
}

// Debt + Debt -> Debt?
impl<T: CheckedOps> ops::Add<Debt<T>> for Debt<T> {
    type Output = Option<Debt<T>>;

    #[inline]
    fn add(self, _rhs: Debt<T>) -> Self::Output {
        Some(Debt(self.0.add_checked(_rhs.0)?))
    }
}

// Credit - Debt -> Asset?
impl<T: CheckedOps> ops::Sub<Debt<T>> for Credit<T> {
    type Output = Option<Asset<T>>;

    #[inline]
    fn sub(self, _rhs: Debt<T>) -> Self::Output {
        let value = self.0.add_checked(_rhs.0)?;
        if value.is_positive() {
            Some(Asset::Credit(Credit(value)))
        } else {
            Some(Asset::Debt(Debt(value)))
        }
    }
}

// Debt + Credit -> Asset?
impl<T: CheckedOps> ops::Add<Credit<T>> for Debt<T> {
    type Output = Option<Asset<T>>;

    #[inline]
    fn add(self, _rhs: Credit<T>) -> Self::Output {
        let value = self.0.add_checked(_rhs.0)?;
        if value.is_positive() {
            Some(Asset::Credit(Credit(value)))
        } else {
            Some(Asset::Debt(Debt(value)))
        }
    }
}

#[derive(Debug, Hash, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(align(8))]
/// Can be either a `Credit` or a `Debt`
///
/// Assets that wraps the same type can be added: a `Debt` is always less then 0 and a `Credit` is
/// always bigger or equal to 0, so only the addition is supported.
///
/// Assets can be multiplied or divided for `i128` and TODO for floating point
///
/// Assets can be created from integer values or tuples TODO link
///
/// The value of an assets can be extract as an `i128` or as a tuple TODO link
pub enum Asset<T: CheckedOps> {
    Debt(Debt<T>),
    Credit(Credit<T>),
}

impl<T: CheckedOps> Asset<T> {

    #[inline]
    /// Return the numeric value of the Asset
    pub fn get_inner(self) -> T {
        match self {
            Self::Debt(x) => x.0,
            Self::Credit(x) => x.0,
        }
    }
}

// Asset + Asset -> Asset?
impl<T: CheckedOps> ops::Add<Asset<T>> for Asset<T> {
    type Output = Option<Asset<T>>;

    #[inline]
    fn add(self, _rhs: Asset<T>) -> Self::Output {
        let lhs = self.get_inner();
        let rhs = _rhs.get_inner();
        let value = lhs.add_checked(rhs)?;
        if value.is_positive() {
            Some(Asset::Credit(Credit(value)))
        } else {
            Some(Asset::Debt(Debt(value)))
        }
    }
}

impl<T: CheckedOps> ops::Mul<i128> for Asset<T> {
    type Output = Option<Asset<T>>;

    #[inline]
    fn mul(self, rhs: i128) -> Self::Output {
        let lhs = self.get_inner();
        let value = lhs.mul_checked(rhs)?;
        if value.is_positive() {
            Some(Asset::Credit(Credit(value)))
        } else {
            Some(Asset::Debt(Debt(value)))
        }
    }
}

impl<T: CheckedOps> ops::Div<i128> for Asset<T> {
    type Output = Option<Asset<T>>;

    #[inline]
    fn div(self, rhs: i128) -> Self::Output {
        let lhs = self.get_inner();
        let value = lhs.div_checked(rhs)?;
        if value.is_positive() {
            Some(Asset::Credit(Credit(value)))
        } else {
            Some(Asset::Debt(Debt(value)))
        }
    }
}

impl<T: TryFrom<i128> + CheckedOps> TryFrom<i128> for Asset<T> {
    type Error = ();

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Asset::Credit(Credit(T::try_from(value).map_err(|_| ())?)))
        } else {
            Ok(Asset::Debt(Debt(T::try_from(value).map_err(|_| ())?)))
        }
    }
}

impl<
    I: TryInto<i128>,
    F: TryInto<u128>,
    T: TryFrom<(i128, F)> + CheckedOps> TryFrom<(I, F)
    > for Asset<T> {

    type Error = ();

    fn try_from(value: (I, F)) -> Result<Self, Self::Error> {
        let integer: i128 = (value.0).try_into().map_err(|_| ())?;
        if integer >= 0 {
            Ok(Asset::Credit(Credit(T::try_from((integer,value.1)).map_err(|_| ())?)))
        } else {
            Ok(Asset::Debt(Debt(T::try_from((integer, value.1)).map_err(|_| ())?)))
        }
    }
}

impl<T: TryFrom<f64> + CheckedOps> TryFrom<f64> for Asset<T> {
    type Error = ();

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value >= 0.0 {
            Ok(Asset::Credit(Credit(T::try_from(value).map_err(|_| ())?)))
        } else {
            Ok(Asset::Debt(Debt(T::try_from(value).map_err(|_| ())?)))
        }
    }
}

impl<V: CheckedOps + TryFrom<i128>> Asset<V> {
    pub fn new<T: Into<i128>>(val: T) -> Option<Self> {
        let val: i128 = val.into();
        Self::try_from(val).ok()
    }
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! new_asset {
    // TODO maybe Value should be defined outside the macro??
    ($mod_name:ident, $frac:tt, $max_value:tt) => {
        mod $mod_name {
            use std::convert::TryFrom;
            use std::convert::TryInto;
            use $crate::asset::Asset;
            use $crate::asset::Credit;
            use $crate::asset::Debt;
            use $crate::asset::get_inner_len;
            use $crate::asset::pow_10;
            use $crate::asset::CheckedOps;
            use super::Fixed;
            use super::IsFixed;
            use super::ArrayWrapper;
            use super::HasBound;
            use std::fmt;
            //const FRAC_B2: u128 = ((332192809489 as u128 * $frac as u128) / pow_10(11)) + 1;
            //const FRAC: usize = $frac;

            const POW: u128 = {
                assert!($frac as u128 <= 38);
                pow_10($frac)
            };
            #[allow(unused_comparisons)]
            const MAX_VAL: i128 = {
                assert!($max_value as i128 >= 0);
                assert!($frac as i128 >= 0);
                assert!((i128::max_value() / $max_value) > POW as i128);
                $max_value * POW as i128
            };
            const LEN: usize = get_inner_len(MAX_VAL);

            pub type Fixed_ = Fixed<LEN, MAX_VAL, POW>;
            impl HasBound<MAX_VAL, LEN> for Fixed_ {}
            impl IsFixed<LEN, MAX_VAL, POW> for Fixed_ {}

            pub fn inspect() {
                println!("POW: {}", POW);
                println!("MAX_VAL: {}", MAX_VAL);
                println!("LEN: {}", LEN);
            }

            #[derive(PartialEq, Copy, Clone)]
            #[repr(align(8))]
            pub struct Value (pub Fixed_);

            impl ArrayWrapper<LEN> for Value {

                fn get_array(self) -> [u8; LEN] {
                    self.0.get_array()
                }

                fn from_array_unchecked(_value: [u8; LEN]) -> Self {
                    panic!("Not safe!")
                }
            }

            impl IsFixed<LEN, MAX_VAL, POW> for Value {}

            // TODO is possible avoid to reimplement everything for the inner value? Maybe
            // implementing Deref?
            impl TryFrom<i128> for Value {
                type Error = ();

                fn try_from(value: i128) -> Result<Self, Self::Error> {
                    Ok(Value(Fixed_::try_from(value)?))
                }
            }

            impl TryFrom<f64> for Value {
                type Error = ();

                fn try_from(value: f64) -> Result<Self, Self::Error> {
                    Ok(Value(Fixed_::try_from(value)?))
                }
            }

            impl<T: TryInto<u128>> TryFrom<(i128, T)> for Value {
                type Error = ();

                fn try_from(value: (i128, T)) -> Result<Self, Self::Error> {
                    let precision: u128 = (value.1).try_into().map_err(|_| ())?;
                    Ok(Value(Fixed_::try_from((value.0, precision))?))
                }
            }

            impl CheckedOps for Value {

                #[inline]
                fn add_checked(self, rhs: Self) -> Option<Self> {
                    Some(Self(self.0.add_checked(rhs.0)?))
                }
                #[inline]
                fn mul_checked(self, rhs: i128) -> Option<Self> {
                    Some(Self(self.0.mul_checked(rhs)?))
                }
                #[inline]
                fn div_checked(self, rhs: i128) -> Option<Self> {
                    Some(Self(self.0.div_checked(rhs)?))
                }
                #[inline]
                fn is_positive(&self) -> bool {
                    self.0.is_positive()
                }
            }

            use super::FixedToInt;
            
            impl<
            
                > FixedToInt for Asset<Value> {
                    fn to_parts(self) -> (i128, i128, u128) {
                        self.get_inner().to_parts()
                    }
                    
                    fn to_int(self) -> i128 {
                        self.to_parts().0
                    }
                }

            impl fmt::Debug for Value {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    let parts = self.to_parts();
                    f.debug_struct("Value")
                     .field("asset name", &stringify!($mod_name))
                     .field("int", &parts.0)
                     .field("fract", &parts.1)
                     .finish()
                }
            }
            
        }
    };
}

#[macro_export]
macro_rules! get_traits {
    () => {
        pub trait FixedToInt {
            fn to_parts(self) -> (i128, i128, u128);
        
            fn to_int(self) -> i128;
        }
        get_fixed!();
    }
}
