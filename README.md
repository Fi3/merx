# Merx

##

**mèr | ce** s. f. [lat. merx mercis]:
[*something meant to be divided, distributable portion.*](https://www.etimo.it/?term=merce&find=Cerca)

[Here](https://github.com/fi3/pinoedino) a test case for Merx.

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

An asset is characterized by a unit (minimum quantity) and an optional upper bound.
The unit is the smallest part of the asset that the software can express.

Addition between assets of the same type are supported out of the box with operator
overloading. Multiplication and division are implemented between assets and numeric
types with operator overloading. So **asset + asset**, **asset * number**,
**asset / number** are valid operations.

An asset can be either a credit or a debit. A debit can only contain negative amounts. A credit can
only contains positive amounts.

Merx expose `Asset` that is a wrapper around a `Debt` or a `Credit` that are wrapper around a
numeric value.
The wrapped numeric value is a dummy fixed value defined in [*/src/fixed.rs*](./src.fixed.rs)

```rust
pub struct <T: NUMERIC>Debt(T);
pub struct <T: NUMERIC>Credit(T);

pub enum Asset<T: NUMERIC> {
    Debt(Debt(T)),NUMERIC
    Credit(Credit(T)),
}
```

The permitted operations are: 
* `Credit - Debt` that is positive_value + negative_value [TODO remove it it should be `Credit + Debt`]
* `Credit + Credit` that is value + value
* `Debt + Credit` that is negative_value + positive_value
* `Debt + Debt` that is negative_value + negative_value
* `Asset + Asset` that is value + value

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

    // With a float also a rounding method must be provided, this because Merx must know what to do
    // with floats with higher precision than the asset. Possible rounding methods are Trunc Floor
    // Ceil Round and they behaved the same as rust's f64 methods with the same names.
    let usd = Usd::try_from((10.87, FloatRounding::Trunc)).unwrap();
    println!("{:#?}", usd);

    // When the source of the float is a text string the best thing to do is to parse the value
    // from a string.
    let usd = Usd::try_from("10.87").unwrap();
    println!("{:#?}", usd);

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

```

## Safety

1. Is impossible to add assets of different types or add an asset with a numeric value.
2. Every operation that concern an asset (add mul div) is checked and fail on incorrect values.
3. Build assets from primitive types is safe [TODO].
4. When the result of an operation is positive we have a `Credit` otherwise we have `Debt`, is not
possible to build a `Credit` with a negative value or a `Debt` with a positive value.
5. The library have 0 dependency.

## Performance

Merx in order to add assets do a checked add. From the benchmark it seems that Merx is a
little faster in doing that than a plain function like the one below.

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

 - [ ] Documentation
 - [ ] Better zero
 - [x] Fix try_from floating point 
 - [ ] Remove support fro `Credit<T> + Debt<T>`
 - [ ] Conversion between Asset Debits and Credits
 - [ ] Error on upper_bound overflow
 - [ ] Set upper bound for asset with no upper bound
 - [ ] Make the inner numeric value generic over ...?
 - [ ] Use the crate fixed as inner type (when it will support generic const)
 - [ ] Impl PartialEq for Asset and all the primitive numeric types
 - [ ] Add error with thiserror
 - [ ] Serde serialize deserialize
 - [ ] Division and multiplication between asset, float and between asset and fixed
 - [ ] Add all standard operations for rationals like truncate floor ecc ecc
 - [ ] Add conversion between assets with exchange rate setted
 - [ ] A lot of public thinghs should be private
 - [ ] Benchmarks
 - [ ] Documentation
 - [ ] Precise modality with bigint insted of fixed point
 - [ ] Add the possibility to define a rounding strategy when an asset is defined
 - [ ] Build asset from primitive values is safe

## License

MIT OR [UNLICENSE](https://unlicense.org/)
