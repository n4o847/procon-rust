use proconio::{fastout, input};

use proconio::marker::Usize1;
use std::cmp::max;
use std::cmp::min;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize, s: i64,
    }
    let mut g = vec![vec![]; n];
    for i in 0..m {
        input! { u: Usize1, v: Usize1, a: i64, b: i64 }
        g[u].push((v, a, b));
        g[v].push((u, a, b));
    }
    let mut h = vec![(0, 0); n];
    for i in 0..n {
        input! { c: i64, d: i64 }
        h[i] = (c, d);
    }
    let mut q = BinaryHeap::new();
    q.push(Reverse((0, min(s, 2500), 0)));
    let mut dp = vec![vec![std::i64::MAX; 2501]; n];
    dp[0][min(s, 2500) as usize] = 0;
    while let Some(Reverse((t, s, i))) = q.pop() {
        let (c, d) = h[i];
        if s + c < 2501 {
            let s0 = s as usize;
            let s1 = (s + c) as usize;
            if dp[i][s1] > dp[i][s0] + d {
                dp[i][s1] = dp[i][s0] + d;
                q.push(Reverse((dp[i][s1], s1 as i64, i)));
            }
        }
        for &(j, a, b) in g[i].iter() {
            if s - a >= 0 {
                let s0 = s as usize;
                let s1 = (s - a) as usize;
                if dp[j][s1] > dp[i][s0] + b {
                    dp[j][s1] = dp[i][s0] + b;
                    q.push(Reverse((dp[j][s1], s1 as i64, j)));
                }
            }
        }
    }
    for i in 1..n {
        let ans = dp[i].iter().fold(std::i64::MAX, |x, y| min(x, *y));
        println!("{}", ans);
    }
}
