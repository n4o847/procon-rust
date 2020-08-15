use proconio::{fastout, input};

use proconio::marker::Usize1;
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize, k: i64,
        pp: [Usize1; n],
        cc: [i64; n],
    }
    let mut ans = std::i64::MIN;
    for s in 0..n {
        let mut i = s;
        let mut t = 0;
        let mut c = 0;
        let mut m = std::i64::MIN;
        loop {
            c += 1;
            i = pp[i];
            t += cc[i];
            m = max(m, t);
            if c == k {
                ans = max(ans, m);
                break;
            }
            if i == s {
                if t <= 0 {
                    ans = max(ans, m);
                } else {
                    t = t * (k / c - 1);
                    m = max(m, t);
                    for _ in 0..(c + k % c) {
                        i = pp[i];
                        t += cc[i];
                        m = max(m, t);
                    }
                    ans = max(ans, m);
                }
                break;
            }
        }
    }
    println!("{}", ans);
}
