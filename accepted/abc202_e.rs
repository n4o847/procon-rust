use proconio::marker::Usize1;
use proconio::{fastout, input};
use std::{collections::VecDeque, vec};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut p: [Usize1; n - 1],
        q: usize,
        ud: [(Usize1, usize); q],
    }
    p.insert(0, 0);
    let mut g = vec![vec![]; n];
    for u in 1..n {
        g[p[u]].push(u);
    }
    let mut dep = vec![0; n];
    let mut seq = vec![];
    let mut idx = vec![0; n];
    let mut rng = vec![0; n];
    let mut que = VecDeque::new();
    que.push_back((0, 0));
    while let Some((u, d)) = que.pop_front() {
        dep[u] = d;
        idx[u] = seq.len();
        seq.push(u);
        rng[idx[u]] = seq.len() + que.len();
        for &v in g[u].iter() {
            que.push_back((v, d + 1));
        }
    }
    rng.push(n);
    let mut dd = vec![vec![0; n]];
    for u in 0..n {
        dd[0][u] = rng[u];
    }
    dd[0].push(n);
    let mut k = 1;
    while k < n {
        let dp = dd.last_mut().unwrap();
        let mut di = vec![0; n];
        for u in 0..n {
            di[u] = dp[dp[u]];
        }
        di.push(n);
        dd.push(di);
        k <<= 1;
    }
    for &(u, d) in ud.iter() {
        if dep[u] > d {
            println!("0");
        } else {
            let mut l = idx[u];
            let mut r = idx[u] + 1;
            let t = d - dep[u];
            for i in 0..32 {
                if t & (1 << i) != 0 {
                    l = dd[i][l];
                    r = dd[i][r];
                }
            }
            // for _ in 0..(d - dep[u]) {
            //     l = rng[l];
            //     r = rng[r];
            //     if l == r {
            //         break;
            //     }
            // }
            println!("{}", r - l);
        }
    }
}
