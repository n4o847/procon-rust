use proconio::{fastout, input};

use std::collections::BTreeMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut m = BTreeMap::new();
    for i in 0..n {
        *m.entry(i as i64 + a[i] as i64).or_insert(0) += 1;
    }
    let mut ans = 0 as i64;
    for i in 0..n {
        ans += *m.entry(i as i64 - a[i] as i64).or_insert(0);
    }
    println!("{}", ans);
}
