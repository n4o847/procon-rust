use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let mut s = HashSet::new();
    for a in 2..=100000 {
        let mut p = a;
        loop {
            p *= a;
            if p > n {
                break;
            }
            s.insert(p);
        }
    }
    let ans = n - s.len() as u64;
    println!("{}", ans);
}
