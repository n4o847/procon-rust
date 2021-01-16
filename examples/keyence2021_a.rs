use std::cmp::max;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
        b: [u64; n],
    }
    let mut ca = a.clone();
    let mut ma = a[0];
    for i in 0..n {
        ma = max(ma, a[i]);
        ca[i] = ma;
    }
    let mut m = a[0] * b[0];
    for i in 0..n {
        m = max(m, ca[i] * b[i]);
        println!("{}", m);
    }
}
