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
    let mut ma = a[0];
    let mut mb = b[0];
    for i in 0..n {
        if ca[i] * b[i] > ma * mb {
            ma = ca[i];
            mb = b[i];
        }
        println!("{}", ma * mb);
    }
}
