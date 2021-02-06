use proconio::marker::Usize1;
use proconio::{fastout, input};
use std::{
    cmp::{min, Reverse},
    collections::BinaryHeap,
};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        abc: [(Usize1, Usize1, u64); m],
    }
    const INF: u64 = std::u64::MAX / 2;
    let mut dist = vec![vec![None; n]; n];
    let mut xyz = vec![];
    for &(a, b, c) in abc.iter() {
        if let Some(d) = dist[a][b] {
            if c >= d {
                continue;
            }
        }
        dist[a][b] = Some(c);
        xyz.push((a, b, c));
    }
    let mut g = vec![vec![]; n];
    for &(a, b, c) in xyz.iter() {
        g[a].push((b, c));
    }
    for s in 0..n {
        let mut que = BinaryHeap::new();
        let mut d = vec![INF; n];
        d[s] = 0;
        que.push(Reverse((0, s)));
        while let Some(Reverse(p)) = que.pop() {
            let v = p.1;
            if d[v] < p.0 {
                continue;
            }
            for &e in g[v].iter() {
                if d[e.0] > d[v] + e.1 {
                    d[e.0] = d[v] + e.1;
                    que.push(Reverse((d[e.0], e.0)));
                }
            }
        }
        let mut ans = INF;
        if let Some(d) = dist[s][s] {
            ans = d;
        }
        for t in 0..n {
            if t == s {
                continue;
            }
            if let Some(x) = dist[t][s] {
                ans = min(ans, d[t] + x);
            }
        }
        if ans >= INF {
            println!("-1");
        } else {
            println!("{}", ans);
        }
    }
}
