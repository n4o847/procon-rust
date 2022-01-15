use proconio::{fastout, input};

use proconio::marker::Chars;
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let mut ans = 0;
    let mut acc = 0;
    for c in s {
        match c {
            'A' | 'C' | 'G' | 'T' => acc += 1,
            _ => acc = 0,
        }
        ans = max(ans, acc);
    }
    println!("{}", ans);
}
