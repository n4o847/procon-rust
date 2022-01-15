use proconio::{fastout, input};

use proconio::marker::Usize1;
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        ab: [(Usize1, Usize1); m],
        k: usize,
        cd: [(Usize1, Usize1); k],
    }
    let mut ans = 0;
    for s in 0..(1 << k) {
        let mut d = vec![false; n];
        for i in 0..k {
            if (s >> i) & 1 == 0 {
                d[cd[i].0] = true;
            } else {
                d[cd[i].1] = true;
            }
        }
        let mut cnt = 0;
        for i in 0..m {
            if d[ab[i].0] && d[ab[i].1] {
                cnt += 1;
            }
        }
        ans = max(ans, cnt);
    }
    println!("{}", ans);
}
