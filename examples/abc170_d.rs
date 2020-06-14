use proconio::{fastout, input};

use std::collections::BTreeMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut m = BTreeMap::new();
    for &x in a.iter() {
        *m.entry(x).or_insert(0) += 1;
    }
    let mut ans = 0;
    for &x in a.iter() {
        let mut ds = vec![];
        let mut i = 1;
        while i * i <= x {
            if x % i == 0 {
                ds.push(i);
                if i * i != x {
                    ds.push(x / i);
                }
            }
            i += 1;
        }
        if ds.iter().all(|&d| {
            let c = m.get(&d).unwrap_or(&0);
            if x == d {
                *c <= 1
            } else {
                *c == 0
            }
        }) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
