// Props:
// 1. asset of the same kind can be summed and subtracted
// 2. asset of differnts kind can not be summed or subtracted
// 3. neither assets of the same or different kind can be multiplied or divided
// 4. build an asset for an amount bigger than upper buond return error
// 5. negative amunts returns Debt<Asset> positive Credit<Asset>
// 6. TODO assets with no upper bound have an upper bound of i128::max_value - frac part
// 7. assets with upper bound have an inner value of fewer bits possible            ###!
// 8. define an asset that can no be represented by an i128 result in a panic
// 9. overflowing operations on Assets result in None
// 8. TODO exchange rates
// 10. TODO operator overloading for `== <= >= !=`                                       ###!
use super::Asset;
use quickcheck::{quickcheck, TestResult};

get_traits!();
new_asset!(test_asset1, 10, 10_000_000_000);
new_asset!(test_asset_low_precision, 2, 18446744073709551615);
new_asset!(test_asset_pass_with_i32, 1, 18446744073709551615);
new_asset!(test_asset_with_upper, 6, 1234);
new_asset!(test_asset_overflow, 0, 2147483647);

#[quickcheck]
fn prop_add_same_kind_assets(amount1: i128, amount2: i128) -> TestResult {
    type MyAsset = Asset<test_asset1::Value>;
    let asset1 = MyAsset::try_from(amount1);
    let asset2 = MyAsset::try_from(amount2);
    match (asset1, asset2) {
        (Ok(asset1), Ok(asset2)) => match asset1 + asset2 {
            None => TestResult::discard(),
            Some(asset) => {
                let expected = amount1 + amount2;
                TestResult::from_bool(expected == asset.to_int())
            }
        },
        _ => TestResult::discard(),
    }
}

#[quickcheck]
fn prop_mul_asset_with_num(amount1: i128, operator: i128) -> TestResult {
    type MyAsset = Asset<test_asset1::Value>;
    let asset1 = MyAsset::try_from(amount1);
    match asset1 {
        Ok(asset1) => {
            let asset2 = asset1 * operator;
            match asset2 {
                Some(asset2) => {
                    let expected = amount1 * operator;
                    TestResult::from_bool(expected == asset2.to_int())
                }
                _ => TestResult::discard(),
            }
        }
        _ => TestResult::discard(),
    }
}

#[quickcheck]
fn prop_add_same_kind_assets_frac(amount1: (i128, i128), amount2: (i128, i128)) -> TestResult {
    type MyAsset = Asset<test_asset1::Value>;
    let asset1 = MyAsset::try_from(amount1);
    let asset2 = MyAsset::try_from(amount2);
    match (asset1, asset2) {
        (Ok(asset1), Ok(asset2)) => match asset1 + asset2 {
            None => TestResult::discard(),
            Some(asset) => {
                let asset1 = amount1.0 * 10_i128.pow((10 - amount1.1) as u32);
                let asset2 = amount2.0 * 10_i128.pow((10 - amount2.1) as u32);
                let expected = asset1 + asset2;
                let parts = asset.to_parts();
                let result = (parts.0 * 10_i128.pow(10)) + parts.1;
                TestResult::from_bool(expected == result)
            }
        },
        _ => TestResult::discard(),
    }
}

#[quickcheck]
fn prop_impossible_build_assets_with_amount_bigger_than_upper1(amount: i128) -> TestResult {
    type MyAsset = Asset<test_asset_with_upper::Value>;
    let asset = MyAsset::try_from(amount);
    if amount > 1234 {
        match asset {
            Err(_) => TestResult::from_bool(true),
            Ok(_) => TestResult::from_bool(false),
        }
    } else {
        TestResult::discard()
    }
}

#[quickcheck]
fn prop_impossible_build_assets_with_amount_bigger_than_upper2(
    amount: i128,
    frac: u8,
) -> TestResult {
    type MyAsset = Asset<test_asset_with_upper::Value>;
    if frac > 38 || (amount % 10_i128.pow(frac as u32) == 0) {
        return TestResult::discard();
    }
    let asset = MyAsset::try_from((amount, frac));
    let actual_amount = amount / 10_i128.pow(frac as u32);
    if actual_amount >= 1234 {
        match asset {
            Err(_) => TestResult::from_bool(true),
            Ok(_) => TestResult::from_bool(false),
        }
    } else {
        TestResult::discard()
    }
}

#[quickcheck]
fn prop_negative_amount_are_debt1(amount: i32) -> TestResult {
    type MyAsset = Asset<test_asset1::Value>;
    let asset = MyAsset::try_from(amount as i128);
    match asset {
        Ok(asset) => {
            if amount < 0 {
                match asset {
                    Asset::Debt(_) => TestResult::from_bool(true),
                    Asset::Credit(_) => TestResult::from_bool(false),
                }
            } else {
                match asset {
                    Asset::Debt(_) => TestResult::from_bool(false),
                    Asset::Credit(_) => TestResult::from_bool(true),
                }
            }
        }
        _ => TestResult::discard(),
    }
}

#[quickcheck]
fn prop_negative_amount_are_debt2(amount1: i32, amount2: i32) -> TestResult {
    type MyAsset = Asset<test_asset_pass_with_i32::Value>;
    let asset1 = MyAsset::try_from(amount1 as i128).unwrap();
    let asset2 = MyAsset::try_from(amount2 as i128).unwrap();
    let amount = amount1 + amount2;
    let asset = (asset1 + asset2).unwrap();
    if amount < 0 {
        match asset {
            Asset::Debt(_) => TestResult::from_bool(true),
            Asset::Credit(_) => TestResult::from_bool(false),
        }
    } else {
        match asset {
            Asset::Debt(_) => TestResult::from_bool(false),
            Asset::Credit(_) => TestResult::from_bool(true),
        }
    }
}

#[quickcheck]
fn prop_error_on_overflow1(amount: i32, operator: i32) -> TestResult {
    type MyAsset = Asset<test_asset_overflow::Value>;
    let asset = MyAsset::try_from(amount as i128).unwrap();
    let mul = asset * operator as i128;
    if amount.checked_mul(operator) == None {
        match mul {
            None => TestResult::from_bool(true),
            Some(_) => TestResult::from_bool(false),
        }
    } else {
        TestResult::discard()
    }
}

#[quickcheck]
fn prop_error_on_overflow2(amount: i32, operator: i32) -> TestResult {
    type MyAsset = Asset<test_asset_overflow::Value>;
    let asset = MyAsset::try_from(amount as i128).unwrap();
    let mul = asset / operator as i128;
    if amount.checked_div(operator) == None {
        match mul {
            None => TestResult::from_bool(true),
            Some(_) => TestResult::from_bool(false),
        }
    } else {
        TestResult::discard()
    }
}

#[test]
fn it_works() {
    type MyAsset = Asset<test_asset_low_precision::Value>;
    let asset1 = MyAsset::try_from((73.5, crate::fixed::FloatRounding::Trunc)).unwrap();
    println!("{:#?}", asset1);
    assert_eq!((73, 50, 100), asset1.to_parts());
}
