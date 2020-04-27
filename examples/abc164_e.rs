use proconio::{fastout, input};

use proconio::marker::Usize1;
use std::cmp::min;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize, s: usize,
    }
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        input! { u: Usize1, v: Usize1, a: usize, b: u64 }
        g[u].push((v, a, b));
        g[v].push((u, a, b));
    }
    let mut h = vec![(0, 0); n];
    for i in 0..n {
        input! { c: usize, d: u64 }
        h[i] = (c, d);
    }
    let mut q = BinaryHeap::new();
    q.push(Reverse((0, min(s, 2500), 0)));
    let mut dp = vec![vec![std::u64::MAX; 2501]; n];
    dp[0][min(s, 2500)] = 0;
    while let Some(Reverse((_, s, i))) = q.pop() {
        let (c, d) = h[i];
        if s + c < 2501 {
            if dp[i][s + c] > dp[i][s] + d {
                dp[i][s + c] = dp[i][s] + d;
                q.push(Reverse((dp[i][s + c], s + c, i)));
            }
        }
        for &(j, a, b) in g[i].iter() {
            if s >= a {
                if dp[j][s - a] > dp[i][s] + b {
                    dp[j][s - a] = dp[i][s] + b;
                    q.push(Reverse((dp[j][s - a], s - a, j)));
                }
            }
        }
    }
    for i in 1..n {
        let ans = dp[i].iter().min().unwrap();
        println!("{}", ans);
    }
}
