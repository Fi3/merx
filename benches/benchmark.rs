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
use merx::{Asset, Debt, Credit, asset::CheckedOps};

get_traits!();
new_asset!(bench_asset_64, 1, 214748364700);
type BenchAsset64 = Asset<bench_asset_64::Value>;
type BenchAsset64Inner = bench_asset_64::Fixed_;

fn add_checked64(a: i64, b: i64) -> Option<i64> {
    a.checked_add(b)
}

pub fn add_checked_buf(le: [u8; 8], ri: [u8; 8]) -> Option<[u8; 8]> {
    let z: i64;
    unsafe {
        let le: [u8; 8] = std::mem::transmute_copy(&le);
        z = <i64>::from_le_bytes(le);
    }
    let y: i64;
    unsafe {
        let ri: [u8; 8] = std::mem::transmute_copy(&ri);
        y = <i64>::from_le_bytes(ri);
    }
    let sum = z.checked_add(y)?.to_le_bytes();
    let sum_: [u8; 8];
    unsafe { sum_ = std::mem::transmute_copy(&sum) }
    Some(sum_)
}

fn compare_64(a: i64, b: i64) -> Option<()> {
    if a.checked_abs()? <= b {
        return Some(());
    }
    None
}

pub fn add_64b_int(c: &mut Criterion) {
    let asset1 = 21474836480_i64;
    let asset2 = 21474837490_i64;
    c.bench_function("add 64 bit int",
                     |b| b.iter(|| add_checked64(black_box(asset1), black_box(asset2))));
}

pub fn add_64b_array_unchecked(c: &mut Criterion) {
    let asset1 = [0,0,0,128,0,0,0,0];
    let asset2 = [101,0,0,128,0,0,0,0];
    c.bench_function("add 64 bit arrays unchecked",
                     |b| b.iter(|| add_checked_buf(black_box(asset1), black_box(asset2))));
}
pub fn compare_64b_int(c: &mut Criterion) {
    let asset = 21474836480_i64;
    let bound = 21474837490_i64;
    c.bench_function("compare 64 bit int",
                     |b| b.iter(|| compare_64(black_box(asset), black_box(bound))));
}

pub fn add_64b_assets_unchecked(c: &mut Criterion) {
    let asset1 = (BenchAsset64Inner::try_from(2147483648).unwrap());
    let asset2 = (BenchAsset64Inner::try_from(2147483749).unwrap());
    c.bench_function("add 64 bit assets unchecked",
                     |b| b.iter(|| BenchAsset64Inner::add_inner(
                             black_box(asset1),
                             black_box(asset2))));
}

pub fn add_64b_assets_inner(c: &mut Criterion) {
    let asset1 = BenchAsset64::try_from(2147483648).unwrap().get_inner().0;
    let asset2 = BenchAsset64::try_from(2147483749).unwrap().get_inner().0;
    c.bench_function("add 64 bit assets checked inner",
                     |b| b.iter(|| black_box(asset1).add_checked(black_box(asset2))));
}

pub fn add_64b_assets_value(c: &mut Criterion) {
    let asset1 = BenchAsset64::try_from(2147483648).unwrap().get_inner();
    let asset2 = BenchAsset64::try_from(2147483749).unwrap().get_inner();
    c.bench_function("add 64 bit assets checked value",
                     |b| b.iter(|| black_box(asset1).add_checked(black_box(asset2))));
}

pub fn assets_get_inner(c: &mut Criterion) {
    let asset1 = BenchAsset64::try_from(2147483648).unwrap();
    let asset2 = BenchAsset64::try_from(2147483749).unwrap();
    c.bench_function("get inner",
                     |b| b.iter(|| (black_box(asset1).get_inner(), black_box(asset2).get_inner())));
}

pub fn assets_get_value(c: &mut Criterion) {
    let asset1 = BenchAsset64::try_from(2147483648).unwrap().get_inner();
    let asset2 = BenchAsset64::try_from(2147483749).unwrap().get_inner();
    c.bench_function("get value",
                     |b| b.iter(|| (black_box(asset1).0, black_box(asset2).0)));
}

pub fn add_64b_assets(c: &mut Criterion) {
    let asset1 = BenchAsset64::try_from(2147483648).unwrap();
    let asset2 = BenchAsset64::try_from(2147483749).unwrap();
    c.bench_function("add 64 bit assets",
                     |b| b.iter(|| black_box(asset1) + black_box(asset2)));
}

criterion_group!(benches,
                 add_64b_int,
                 //add_64b_array_unchecked,
                 compare_64b_int,
                 //add_64b_assets_unchecked,
                 //add_64b_assets_inner,
                 //add_64b_assets_value,
                 //assets_get_inner,
                 //assets_get_value,
                 add_64b_assets,
                 );
criterion_main!(benches);
