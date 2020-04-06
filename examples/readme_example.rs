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

