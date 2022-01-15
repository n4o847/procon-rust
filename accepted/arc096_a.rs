use proconio::{fastout, input};

use std::cmp::{max, min};

#[fastout]
fn main() {
    input! {
        a: i64, b: i64, c: i64, x: i64, y: i64,
    }
    let mut ans = a * x + b * y;
    for z in 0..=max(x, y) {
        ans = min(ans, a * max(0, x - z) + b * max(0, y - z) + c * z * 2);
    }
    println!("{}", ans);
}
