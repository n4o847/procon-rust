use proconio::{fastout, input};

use std::cmp::max;

#[fastout]
fn main() {
    input! {
        mut n: i64, m: usize,
        a: [i64; m]
    }
    for x in a.iter() {
        n -= x;
    }
    println!("{}", max(n, -1));
}
