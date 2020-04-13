use proconio::{fastout, input};

use num::integer::gcd;

#[fastout]
fn main() {
    input! {
        k: u64
    }
    let mut ans = 0;
    for a in 1..=k {
        for b in 1..=k {
            for c in 1..=k {
                ans += gcd(a, gcd(b, c));
            }
        }
    }
    println!("{}", ans);
}
