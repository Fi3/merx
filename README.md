# Merx

##

**mèr | ce** s. f. [lat. merx mercis]:
[*something meant to be divided, distributable portion.*](https://www.etimo.it/?term=merce&find=Cerca)

##

Merx is a library useful to talk about quantities in a safe way and with an eye on the
performance. It is inspired by this
[article](https://tech.fpcomplete.com/blog/safe-decimal-right-on-the-money) from
[fpcomplete](https://tech.fpcomplete.com/tech).

## How it work:
Merx let you defines assets. An asset is everything that has an amount and can be divided, for
example an asset could be a currency, a commodity, (a physical quantity?) ecc ecc

An asset is charaterized by a minimum quantity (unit), the smallest part of the asset that you
can have or that it make sense to talk about. Sometimes (a lot) is possible to think of an upper
bound for the asset, so we can define it and make impossible to create values that are too big.

Addition and subtraction between assets of the same type are supported out of the box with operator
overloading. Multiplication and division are implemented between assets and numeric
types with operator overloading.

An asset can not owe a negative amount but can be either a credit or a debit, so that it must
be explicitly stated when a negative amount is an option.

Every time that an asset's amount overcome the upper bound [an Error is returned][TODO]. If an
asset do not specify an upper bound, then [`i128` is used as inner value of the asset and the upper
bound is set to `i128::max_value()`][TODO]. 

When we add/subtract assets or assets are multiplied by a number, the result is checked for
overflows, and in case of an overflow an [Error is returned][TODO].

An exchange rate between two different assets can be set, then is possible to convert (explicitly)
one asset in the other and vice versa.

The library expose:
```rust
pub strruct Debt(FIXED);
pub strruct Credit(FIXED);

pub enum Asset<T> {
    Debt(Debt(T)),
    Credit(Credit(T)),
}
```

The permitted operations are: `Credit - Debt` `Credit + Credit` `Debt + Credit` `Debt + Debt` 
`Asset + Asset`


## Example
```rust
use merx::{Asset, Credit, Debit, Error}

new_asset!(bitcoin, 100000000, 21_000_000);
new_asset!(usd, 100, 14_000_000_000_000);

// TODO asset_change!("USD", "Bitcoin", get_change);

type Bitcoin = Asset<bitcoin::Value>;
type Usd = Asset<usd::Value>;

// Adding assets of type T return an asset of type T
fn add_assets<T: Asset>(x: T, y: T) -> Option<Asset<T>> {
    x + y
};

// Adding credits can only result in a Credit
fn add_credits<T>(x: Credit<T>, y: Credit<T>) -> Option<Credit<T>> {
    x + y
}

// Adding debts can only result in a Debt
fn add_credits<T>(x: Debt<T>, y: Debt<T>) -> Option<Debt<T>> {
    x + y
}

// Adding debts can only result in a Debt
fn add_credits<T>(x: Debt<T>, y: Debt<T>) -> Option<Debt<T>> {
    x + y
}

fn interests<T: Asset, X: Num>(asset: T, periods: X, interest_rate: X) -> Option<Asset<T>> {
    // TODO The user can decide how precise it want to be choosing the type of the factor*
    let factor = i128::checked_pow((1 + interest_rate), periods)?;
    asset * factor
}

fn main() {
    let tot_amount = Bitcoin::from_num(679, 1); // -> 67.9 btc
    let to_pay = Bitcoin::try_from(-79, 1); // -> -7.9 btc
    let remain = (tot_amount - to_pay)?;

    // TODO smouthly conversion
    //let x: USD = match remain {
    //    Credit(x) => interests(USD::from(x), 12, 3);
    //    Debit(x) => interests(USD::from(x), 12, 3);
    //};
}
// * if the precision of the operator is less than the one of the Asset the Asset's precision is used
```

## Saefty

1. Is impossible to add assets of differnet types or asset with primitive numeric values.
2. Every operation that concern assets (add mul div) is checked and fail on incorrect values.
3. Build assets from primitive types is save [TODO].
4. The library have 0 dependency.
5. When the result of an operation is positive we have a `Credit` otherwise we have `Debt`, is not
possible to build a `Credit` with a negative value or a `Debt` with a positive value.

## Performance

Adding assets mean do a `checked_add` and check if the value is less or equal than max. The 
library seems to be a little faster in doing that than a plain function like the below:

```rust
fn checked_add_and_compare_64(a: i64, b: i64, max: i64) -> Option<i64> {
    let sum = a.checked_add(b)?;
    if sum.checked_abs()? <= max {
        return Some(sum);
    }
    return None
}
```

```
Benchmarking add 64 bit int
Benchmarking add 64 bit int: Warming up for 3.0000 s
Benchmarking add 64 bit int: Collecting 100 samples in estimated 5.0000 s (2.5B iterations)
Benchmarking add 64 bit int: Analyzing
add 64 bit int          time:   [2.0150 ns 2.0205 ns 2.0281 ns]
                        change: [-1.3576% -0.5189% +0.0981%] (p = 0.19 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe

Benchmarking add 64 bit assets
Benchmarking add 64 bit assets: Warming up for 3.0000 s
Benchmarking add 64 bit assets: Collecting 100 samples in estimated 5.0000 s (2.5B iterations)
Benchmarking add 64 bit assets: Analyzing
add 64 bit assets       time:   [2.0138 ns 2.0168 ns 2.0210 ns]
                        change: [-1.0088% -0.3617% +0.1265%] (p = 0.25 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe
```

This it happen also when assets are multiplied or divided. I think that the library is not
slow but I can't justify the above numbers. I'm not confident in these benchmarks and in my
benchmarking ability, the benchmarks must be reviewed.



## Precision

[TODO]

## Alternatives

I couldn't be able to find any library like merx.
If you need something to do fixed arithmetic in rust there is
(fixed)[https://docs.rs/fixed/0.5.4/fixed/].
Money libraries [TODO]
If you need to manipulate units of measure there is [yaiouom](https://github.com/Yoric/yaiouom)

## Todo

 - [ ] When fixed support generic const use Fixed as inner type, or export ./fixed.rs in a separate crate
 - [ ] Precise mod with bigint insted of fixed point
 - [ ] Impl PartialEq for Asset and all the primitive numeric types
 - [ ] Add error with thiserror
 - [ ] Serde serialize deserialize
 - [ ] Division and multiplication between asset, float and between asset and fixed
 - [ ] Add all standard operations for rationals like truncate floor ecc ecc
 - [ ] Add conversion between assets

## License

[UNLICENSE](https://unlicense.org/)
