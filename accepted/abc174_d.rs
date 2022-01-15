use proconio::{fastout, input};

use proconio::marker::Chars;
use std::cmp::{max, min};

#[fastout]
fn main() {
    input! {
        n: usize,
        cc: Chars,
    }
    let mut ws = vec![0; n + 1];
    let mut rs = vec![0; n + 1];
    for i in 0..n {
        ws[i + 1] = ws[i] + if cc[i] == 'W' { 1 } else { 0 };
        rs[i + 1] = rs[i] + if cc[i] == 'R' { 1 } else { 0 };
    }
    let rn = rs[n];
    let mut ans = n + 1;
    for i in 0..=n {
        ans = min(ans, max(ws[i], rn - rs[i]));
    }
    println!("{}", ans);
}
