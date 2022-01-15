use proconio::{fastout, input};

use proconio::marker::Usize1;
use std::cmp::{min, Reverse};
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        ab: [(Usize1, Usize1); m],
        k: usize,
        c: [Usize1; k],
    }
    const INF: u64 = std::u64::MAX / 2;
    let mut g = vec![vec![]; n];
    for &(a, b) in ab.iter() {
        g[a].push(b);
        g[b].push(a);
    }
    let mut f = vec![vec![0; k]; k];
    for i in 0..k {
        let s = c[i];
        let mut d = vec![INF; n];
        let mut h = BinaryHeap::new();
        d[s] = 0;
        h.push(Reverse((0, s)));
        while let Some(Reverse(p)) = h.pop() {
            let v = p.1;
            if d[v] < p.0 {
                continue;
            }
            for &w in g[v].iter() {
                if d[w] > d[v] + 1 {
                    d[w] = d[v] + 1;
                    h.push(Reverse((d[w], w)));
                }
            }
        }
        for j in 0..k {
            f[i][j] = d[c[j]];
        }
    }

    let mut ans = INF;
    for s in 0..k {
        let mut dp = vec![vec![INF; k]; 1 << k];
        dp[1 << s][s] = 0;
        for s in 0..(1 << k) {
            for i in 0..k {
                if s & (1 << i) != 0 {
                    for j in 0..k {
                        if dp[s | (1 << j)][j] > dp[s][i] + f[i][j] {
                            dp[s | (1 << j)][j] = dp[s][i] + f[i][j];
                        }
                    }
                }
            }
        }
        ans = min(ans, *dp[(1 << k) - 1].iter().min().unwrap());
    }
    println!("{}", if ans >= INF { -1 } else { ans as i64 + 1 });
}
