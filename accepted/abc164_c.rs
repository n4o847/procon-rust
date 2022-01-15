use proconio::{fastout, input};

use std::collections::BTreeSet;
use std::iter::FromIterator;

#[fastout]
fn main() {
    input! {
        n: usize,
        ss: [String; n]
    }
    let s = BTreeSet::from_iter(ss.iter());
    println!("{}", s.len());
}
