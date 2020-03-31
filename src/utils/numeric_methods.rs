use std::convert::TryFrom;

pub fn checked_int_from_f64(max_val: u128, precision: u8, val: f64) -> Option<i128> {
    if !val.is_finite() {
        return None;
    }
    let val = val * 10.0_f64.powi(precision as i32);
    if val > max_val as f64 {
        return None;
    }
    if val.fract() == 0.0 {
	Some(val as i128)
    } else {
        None
    }
}


pub fn round_value_up(max_val: u128, precision: u8, val: f64) -> Option<i128> {
    if !val.is_finite() {
        return None;
    }
    let val = val * 10.0_f64.powi(precision as i32).ceil();
    if val > max_val as f64 {
        return None;
    }
    Some(val as i128)
}


pub fn round_value_down(max_val: u128, precision: u8, val: f64) -> Option<i128> {
    if !val.is_finite() {
        return None;
    }
    let val = val * 10.0_f64.powi(precision as i32).floor();
    if val > max_val as f64 {
        return None;
    }
    Some(val as i128)
}

macro_rules! add_buffers {
    ($fn_name:tt, $int_type:ty, $len:tt) => {
        #[inline]
        pub fn $fn_name<'a, const LEN: usize>(le: [u8; LEN], ri: [u8; LEN]) -> Option<[u8; LEN]> {
            let z: $int_type;
            unsafe {
                let le: [u8; $len] = std::mem::transmute_copy(&le);
                z = <$int_type>::from_le_bytes(le);
            }
            let y: $int_type;
            unsafe {
                let ri: [u8; $len] = std::mem::transmute_copy(&ri);
                y = <$int_type>::from_le_bytes(ri);
            }
            let sum = z.checked_add(y)?.to_le_bytes();
            let sum_: [u8; LEN];
            unsafe { sum_ = std::mem::transmute_copy(&sum) }
            Some(sum_)
        }
    };
}

add_buffers!(add_buffers_32, i32, 4);
add_buffers!(add_buffers_64, i64, 8);
add_buffers!(add_buffers_128, i128, 16);

macro_rules! buffer_is_less_or_equal {
    ($fn_name:tt, $int_type:ty, $len:tt) => {
        #[inline]
        pub fn $fn_name<const LEN: usize>(val: [u8; LEN], max: $int_type) -> Option<()> {
            let z: $int_type;
            unsafe {
                let val: [u8; $len] = std::mem::transmute_copy(&val);
                z = <$int_type>::from_le_bytes(val);
            }
            if z.checked_abs()? <= max {
                return Some(());
            }
            None
        }
    };
}

buffer_is_less_or_equal!(buffer_is_less_or_equal_32, i32, 4);
buffer_is_less_or_equal!(buffer_is_less_or_equal_64, i64, 8);
buffer_is_less_or_equal!(buffer_is_less_or_equal_128, i128, 16);

macro_rules! ord {
    ($fn_name:tt, $int_type:ty, $len:tt) => {
        #[inline]
        pub fn $fn_name<const LEN: usize>(lhs: [u8; LEN], rhs: [u8; LEN]) -> std::cmp::Ordering {
            let z: $int_type;
            unsafe {
                let lhs: [u8; $len] = std::mem::transmute_copy(&lhs);
                z = <$int_type>::from_le_bytes(lhs);
            }
            let y: $int_type;
            unsafe {
                let rhs: [u8; $len] = std::mem::transmute_copy(&rhs);
                y = <$int_type>::from_le_bytes(rhs);
            }
            if z == y {
                std::cmp::Ordering::Equal
            } else if z >= y {
                std::cmp::Ordering::Greater
            } else if z <= y {
                std::cmp::Ordering::Less
            } else {
                panic!("TODO")
            }
        }
    };
}

ord!(ord_32, i32, 4);
ord!(ord_64, i64, 8);
ord!(ord_128, i128, 16);

macro_rules! buffer_is_positive {
    ($fn_name:tt, $int_type:ty, $len:tt) => {
        #[inline]
        pub fn $fn_name<const LEN: usize>(val: [u8; LEN]) -> bool {
            let z: $int_type;
            unsafe {
                let val: [u8; $len] = std::mem::transmute_copy(&val);
                z = <$int_type>::from_le_bytes(val);
            }
            z >= 0
        }
    };
}

buffer_is_positive!(buffer_is_positive_32, i32, 4);
buffer_is_positive!(buffer_is_positive_64, i64, 8);
buffer_is_positive!(buffer_is_positive_128, i128, 16);

#[inline]
pub fn buffer_is_negative<const LEN: usize>(val: [u8; LEN]) -> bool {
    let most_significant = val[LEN - 1];
    if most_significant.leading_zeros() == 0 {
        return true;
    };
    false
}

macro_rules! mul_buffer {
    ($fn_name:tt, $int_type:ty, $len:tt) => {
        #[inline]
        pub fn $fn_name<'a, const LEN: usize>(le: [u8; LEN], ri: $int_type) -> Option<[u8; LEN]> {
            let z: $int_type;
            unsafe {
                let le: [u8; $len] = std::mem::transmute_copy(&le);
                z = <$int_type>::from_le_bytes(le);
            }
            let mul = z.checked_mul(ri)?.to_le_bytes();
            let mul_: [u8; LEN];
            unsafe {
                mul_ = std::mem::transmute_copy(&mul);
            }
            Some(mul_)
        }
    };
}
mul_buffer!(mul_buffer_32, i32, 4);
mul_buffer!(mul_buffer_64, i64, 8);
mul_buffer!(mul_buffer_128, i128, 16);

macro_rules! buffer_to_parts {
    ($fn_name:tt, $int_type:ty, $len:tt) => {
        pub fn $fn_name<'a, const LEN: usize, const POW: u128>(
            buf: [u8; LEN],
        ) -> ($int_type, $int_type, u128) {
            let z: $int_type;
            unsafe {
                let buf: [u8; $len] = std::mem::transmute_copy(&buf);
                z = <$int_type>::from_le_bytes(buf);
            }
            let int = z / <$int_type>::try_from(POW).unwrap();
            let frac = z % <$int_type>::try_from(POW).unwrap();
            (int, frac, POW)
        }
    };
}
buffer_to_parts!(buffer_to_parts_32, i32, 4);
buffer_to_parts!(buffer_to_parts_64, i64, 8);
buffer_to_parts!(buffer_to_parts_128, i128, 16);

macro_rules! div_buffer {
    ($fn_name:ident, $int_type:ty, $len:tt) => {
        #[inline]
        pub fn $fn_name<'a, const LEN: usize>(lhs: [u8; LEN], rhs: $int_type) -> Option<[u8; LEN]> {
            let z: $int_type;
            unsafe {
                let lhs: [u8; $len] = std::mem::transmute_copy(&lhs);
                z = <$int_type>::from_le_bytes(lhs);
            }
            let div = z.checked_div(rhs)?;
            let div = div.to_le_bytes();
            let div_: [u8; LEN];
            unsafe {
                div_ = std::mem::transmute_copy(&div);
            }
            Some(div_)
        }
    };
}

div_buffer!(div_buffer_32, i32, 4);
div_buffer!(div_buffer_64, i64, 8);
div_buffer!(div_buffer_128, i128, 16);
