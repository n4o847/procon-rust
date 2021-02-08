use num_integer::gcd;
use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut h = HashMap::new();
    for &x in a.iter() {
        let mut d = vec![];
        for k in 1.. {
            if x % k == 0 {
                d.push(k);
                if k * k != x {
                    d.push(x / k);
                }
            }
            if k * k >= x {
                break;
            }
        }
        for &k in d.iter() {
            if let Some(&y) = h.get(&k) {
                h.insert(k, gcd(x, y));
            } else {
                h.insert(k, x);
            }
        }
    }
    let m = a.iter().min().unwrap();
    let ans = h.iter().filter(|&(k, v)| k == v && k <= m).count();
    println!("{}", ans);
}
