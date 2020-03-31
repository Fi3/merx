# Merx

##

**mèr | ce** s. f. [lat. merx mercis]:
[*something meant to be divided, distributable portion.*](https://www.etimo.it/?term=merce&find=Cerca)

##

// TODO specify that you can only add assets
/fn prop_sub_same_kind_assets(amount1: i128, amount2: i128) -> TestResult {

Merx is a library useful to talk about quantities in a safe way and with an eye on the
performance. It is inspired by this
[article](https://tech.fpcomplete.com/blog/safe-decimal-right-on-the-money) from
[fpcomplete](https://tech.fpcomplete.com/tech).

## How it work:
Merx let you defines assets. An asset is everything that has an amount and can be divided, for
example ...

An asset is charaterized by a minimum quantity (unit), the smallest part of the asset that you
can have or that it make sense to talk about. Sometimes (a lot) is possible to think of an upper
bound for the asset, so we can define it and make impossible to create values that are too big.

Addition and subtraction between assets of the same type are supported out of the box with operator
overloading. Multiplication and division are implemented between assets and numeric
types with operator overloading.

An asset can not owe a negative amount but can be either a credit or a debit, so that it must
be explicitly stated when a negative amount is an option.

Every time that an asset's amount overcome the upper bound an Error is returned.If an asset do not
specify an upper bound, then `i128` is used as inner value of the asset and the upper bound is set
to `i128::max_value()`. 

When we add/subtract assets or assets are multiplied by a number, the result is checked for
overflows, and in case of an overflow an Error is returned.

An exchange rate between two different assets can be set, then is possible to convert (explicitly)
one asset in the other and vice versa.

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

## Performance

## Precision

## Alternatives

Numeric libraries, Money libraries, units libraries
https://github.com/Yoric/yaiouom

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
