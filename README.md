# Merx

##

**mèr | ce** s. f. [lat. merx mercis]:
[*something meant to be divided, distributable portion.*](https://www.etimo.it/?term=merce&find=Cerca)

##

**Pre-beta version** if you think that merx can be useful or you like it, [here](https://users.rust-lang.org/t/help-annuncment-merx-let-you-define-decimal-safe-types/40531) a
thread about which direction this library should take.

##

rustc version >= (1edd389cc 2020-03-23)

##

Merx is a library useful to talk about quantities in a safe way and with an eye on the
performance. It is inspired by this
[article](https://tech.fpcomplete.com/blog/safe-decimal-right-on-the-money) from
[fpcomplete](https://tech.fpcomplete.com/tech).

## How it work:
Merx let you defines assets. An asset is everything that has an amount and can be divided, for
example an asset could be a currency, a commodity, (a physical quantity?) ecc ecc

An asset is characterized by a minimum quantity (unit), the smallest part of the asset that you
can have or that it make sense to talk about. Sometimes (a lot) is possible to think of an upper
bound for the asset, so we can define it and make impossible to create values that are too big.

Addition and subtraction between assets of the same type are supported out of the box with operator
overloading. Multiplication and division are implemented between assets and numeric
types with operator overloading.

An asset can not owe a negative amount but can be either a credit or a debit, so that it must
be explicitly stated when a negative amount is an option. (If a function accept Asset it means
that work on both positive or negative amounts, but when it accept Credit or Debt you can be sure
that it works only on positive or only on negative amounts.

Every time that an asset's amount overcome the upper bound [an Error is returned][TODO]. If an
asset do not specify an upper bound, then [`i128` is used as inner value of the asset and the upper
bound is set to `i128::max_value()`][TODO]. 

When we add/subtract assets or assets are multiplied by a number, the result is checked for
overflows, and in case of an overflow an [Error is returned][TODO].

TODO
An exchange rate between two different assets can be set, then is possible to convert (explicitly)
one asset in the other and vice versa.

Merx expose `Asset` a wrapper around a `Debt` or a `Credit` that are wrapper around a numeric value,
for now they work only with a dummy internal fixed value, but I want to make it generic so that it
can be selected when the asset is defined.
```rust
pub struct <T: NUMERIC>Debt(T);
pub struct <T: NUMERIC>Credit(T);

pub enum Asset<T: NUMERIC> {
    Debt(Debt(T)),NUMERIC
    Credit(Credit(T)),
}
```

The permitted operations are: `Credit - Debt` `Credit + Credit` `Debt + Credit` `Debt + Debt` 
`Asset + Asset`. Because `Credit` can not own a negative value, `Debt` can not owe a positive value
and `Asset` is either a `Credit` or a `Debt`, is not clear what addition and subtraction between
assets means in my opinion the possibilities that make more sense are:
1. only add(A, B) exist:
`Asset(x) + Asset(-y) = Asset(x +  (-y))`
2. add(A, B) == sub(A, B) 
`Asset(x) + Asset(-y) = Asset(x +  (-y)) && Asset(x) - Asset(-y) = Asset(x + (-y))`
3. add(A, B) == sub(A, -B)
`Asset(x) + Asset(-y) = Asset(x +  (-y)) && Asset(x) - Asset(-y) = Asset(x - (-y))`

For the moment add and sub behave like (1), because I think that is the less error prone behavior
and the merx main goal is safety, by the way usability is also important and I think that (1) is
not very usable.

## Example
```rust
#[macro_use]
extern crate merx;
use merx::{Asset, Debt, Credit, asset::CheckedOps};

get_traits!();

// Create a new asset called bitcoin with 8 decimal digits and a max value of 21 million of units
new_asset!(bitcoin, 8, 21_000_000);
// Create a new asset called usd with 2 decimal digits and a max value of 14_000_000_000_000 units
new_asset!(usd, 2, 14_000_000_000_000);

type Bitcoin = Asset<bitcoin::Value>;
type Usd = Asset<usd::Value>;

fn main() {
    // A tuple that define a decimal value as (mantissa, decimal part)
    let tot_amount = (679, 1); // -> 67.9
    let tot_amount = Bitcoin::try_from(tot_amount).unwrap();
    let to_pay = Bitcoin::try_from(-29).unwrap();
    let remain = (tot_amount + to_pay).unwrap();
    println!("{:#?}", remain);

    // TODO smouthly conversion
    //let x: USD = match remain {
    //    Credit(x) => interests(USD::from(x), 12, 3);
    //    Debt(x) => interests(USD::from(x), 12, 3);
    //};
}

// You can define function over generic assets:

// Adding assets of type T return an asset of type T
fn add_assets<T: CheckedOps>(x: Asset<T>, y: Asset<T>) -> Option<Asset<T>> {
    x + y
}

// Adding credits can only result in a Credit
fn add_credits<T: CheckedOps>(x: Credit<T>, y: Credit<T>) -> Option<Credit<T>> {
    x + y
}

// Adding debts can only result in a Debt
fn add_debts<T: CheckedOps>(x: Debt<T>, y: Debt<T>) -> Option<Debt<T>> {
    x + y
}

// Adding debts can only result in a Debt
fn add_debts2<T: CheckedOps>(x: Debt<T>, y: Debt<T>) -> Option<Debt<T>> {
    x + y
}
```

## Safety

1. Is impossible to add assets of different types or asset with numeric values.
2. Every operation that concern assets (add mul div) is checked and fail on incorrect values.
3. Build assets from primitive types is safe [TODO].
4. When the result of an operation is positive we have a `Credit` otherwise we have `Debt`, is not
possible to build a `Credit` with a negative value or a `Debt` with a positive value.
5. The library have 0 dependency.

## Performance

Internally adding assets mean do a `checked_add` and check if the value is less or equal than max. The 
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

This happens also when assets are multiplied or divided. I think that the library is not
slow but I can not justify the above numbers. I'm not confident in these benchmarks and in my
benchmarking ability, the benchmarks must be reviewed.

## Precision

[TODO]

## Alternatives

The similest crate that I have been able to find is [commodity](https://crates.io/crates/commodity),
merx is different from commodity in ... TODO

Below a list of crates that solve problems that are similar or related at the one solved by merx.

**Fixed point arithmetic**
* [fixed](https://docs.rs/fixed/0.5.4/fixed/).
*  ... TODO

**Decimal**
* [rust_decimal](https://crates.io/crates/rust_decimal)

**Money crates**
[TODO]

**Units crates**
* [yaiouom](https://github.com/Yoric/yaiouom)
* ... TODO

## Todo

 - [ ] Use the crate fixed as inner type (when it will support generic const)
 - [ ] Impl PartialEq for Asset and all the primitive numeric types
 - [ ] Add error with thiserror
 - [ ] Serde serialize deserialize
 - [ ] Division and multiplication between asset, float and between asset and fixed
 - [ ] Add all standard operations for rationals like truncate floor ecc ecc
 - [ ] Add conversion between assets
 - [ ] A lot of public thinghs should be private
 - [ ] Benchmarks
 - [ ] Documentation
 - [ ] Precise modality with bigint insted of fixed point
 - [ ] Add the possibility to define a rounding strategy when an asset is defined

## License

MIT OR [UNLICENSE](https://unlicense.org/)
