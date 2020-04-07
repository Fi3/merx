#![feature(const_fn)]
#![feature(const_panic)]
#![feature(const_if_match)]
#![feature(const_generics)]
#![feature(const_loop)]
#![allow(incomplete_features)]
#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate merx;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use merx::{asset::CheckedOps, Asset, Credit, Debt};

get_traits!();
new_asset!(bench_asset_64, 1, 2147483647000);
new_asset!(bench_asset_32, 1, 1000000);
new_asset!(bench_asset_128, 10, 2147483647000);
type BenchAsset64 = Asset<bench_asset_64::Value>;
type BenchAsset32 = Asset<bench_asset_32::Value>;
type BenchAsset128 = Asset<bench_asset_128::Value>;

fn checked_add_and_compare_64(a: i64, b: i64, max: i64) -> Option<i64> {
    let sum = a.checked_add(b)?;
    if sum.checked_abs()? <= max {
        return Some(sum);
    }
    return None;
}

fn checked_mul_and_compare_64(a: i64, b: i64, max: i64) -> Option<i64> {
    let res = a.checked_mul(b)?;
    if res.checked_abs()? <= max {
        return Some(res);
    }
    return None;
}

fn checked_div_and_compare_64(a: i64, b: i64, max: i64) -> Option<i64> {
    let res = a.checked_div(b)?;
    if res.checked_abs()? <= max {
        return Some(res);
    }
    return None;
}
fn checked_add_and_compare_32(a: i32, b: i32, max: i32) -> Option<i32> {
    let sum = a.checked_add(b)?;
    if sum.checked_abs()? <= max {
        return Some(sum);
    }
    return None;
}

fn checked_mul_and_compare_32(a: i32, b: i32, max: i32) -> Option<i32> {
    let res = a.checked_mul(b)?;
    if res.checked_abs()? <= max {
        return Some(res);
    }
    return None;
}

fn checked_div_and_compare_32(a: i32, b: i32, max: i32) -> Option<i32> {
    let res = a.checked_div(b)?;
    if res.checked_abs()? <= max {
        return Some(res);
    }
    return None;
}
fn checked_add_and_compare_128(a: i128, b: i128, max: i128) -> Option<i128> {
    let sum = a.checked_add(b)?;
    if sum.checked_abs()? <= max {
        return Some(sum);
    }
    return None;
}

fn checked_mul_and_compare_128(a: i128, b: i128, max: i128) -> Option<i128> {
    let res = a.checked_mul(b)?;
    if res.checked_abs()? <= max {
        return Some(res);
    }
    return None;
}

fn checked_div_and_compare_128(a: i128, b: i128, max: i128) -> Option<i128> {
    let res = a.checked_div(b)?;
    if res.checked_abs()? <= max {
        return Some(res);
    }
    return None;
}

pub fn add_64b_int(c: &mut Criterion) {
    let asset1 = 21474836480_i64;
    let asset2 = 21474837490_i64;
    let bound = 214748374900_i64;
    c.bench_function("add 64 bit int", |b| {
        b.iter(|| {
            checked_add_and_compare_64(black_box(asset1), black_box(asset2), black_box(bound))
        })
    });
}

pub fn add_64b_assets(c: &mut Criterion) {
    let asset1 = BenchAsset64::try_from(2147483648).unwrap();
    let asset2 = BenchAsset64::try_from(2147483749).unwrap();
    c.bench_function("add 64 bit assets", |b| {
        b.iter(|| black_box(asset1) + black_box(asset2))
    });
}

pub fn mul_64b_int(c: &mut Criterion) {
    let asset1 = 21474836480_i64;
    let operator = 2147483_i64;
    let bound = 214748374900_i64;
    c.bench_function("mul 64 bit int", |b| {
        b.iter(|| {
            checked_mul_and_compare_64(black_box(asset1), black_box(operator), black_box(bound))
        })
    });
}

pub fn mul_64b_assets(c: &mut Criterion) {
    let asset1 = BenchAsset64::try_from(2147483648).unwrap();
    let operator = 2147483_i128;
    c.bench_function("mul 64 bit assets", |b| {
        b.iter(|| black_box(asset1) * black_box(operator))
    });
}

pub fn div_64b_int(c: &mut Criterion) {
    let asset1 = 21474836480_i64;
    let operator = 2147483_i64;
    let bound = 214748374900_i64;
    c.bench_function("div 64 bit int", |b| {
        b.iter(|| {
            checked_div_and_compare_64(black_box(asset1), black_box(operator), black_box(bound))
        })
    });
}

pub fn div_64b_assets(c: &mut Criterion) {
    let asset1 = BenchAsset64::try_from(2147483648).unwrap();
    let operator = 2147483_i128;
    c.bench_function("div 64 bit assets", |b| {
        b.iter(|| black_box(asset1) / black_box(operator))
    });
}

pub fn add_32b_int(c: &mut Criterion) {
    let asset1 = 214748_i32;
    let asset2 = 214740_i32;
    let bound = 2147483_i32;
    c.bench_function("add 32 bit int", |b| {
        b.iter(|| {
            checked_add_and_compare_32(black_box(asset1), black_box(asset2), black_box(bound))
        })
    });
}

pub fn add_32b_assets(c: &mut Criterion) {
    let asset1 = BenchAsset32::try_from(214748).unwrap();
    let asset2 = BenchAsset32::try_from(214740).unwrap();
    c.bench_function("add 32 bit assets", |b| {
        b.iter(|| black_box(asset1) + black_box(asset2))
    });
}

pub fn add_128b_int(c: &mut Criterion) {
    let asset1 = 21474836480_i128;
    let asset2 = 21474837490_i128;
    let bound = 214748374900_i128;
    c.bench_function("add 128 bit int", |b| {
        b.iter(|| {
            checked_add_and_compare_128(black_box(asset1), black_box(asset2), black_box(bound))
        })
    });
}

pub fn add_128b_assets(c: &mut Criterion) {
    let asset1 = BenchAsset128::try_from(2147483648).unwrap();
    let asset2 = BenchAsset128::try_from(2147483749).unwrap();
    c.bench_function("add 128 bit assets", |b| {
        b.iter(|| black_box(asset1) + black_box(asset2))
    });
}

criterion_group!(
    benches64,
    add_64b_int,
    add_64b_assets,
    mul_64b_int,
    mul_64b_assets,
    div_64b_int,
    div_64b_assets,
);
criterion_group!(benches32, add_32b_int, add_32b_assets,);
criterion_group!(benches128, add_128b_int, add_128b_assets,);
criterion_main!(benches64, benches32, benches128);
