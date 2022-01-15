use std::cmp::max;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
        lr: [(i64, i64); t],
    }
    for &(l, r) in lr.iter() {
        let a = max(r - l * 2 + 1, 0);
        println!("{}", a * (a + 1) / 2);
    }
}
