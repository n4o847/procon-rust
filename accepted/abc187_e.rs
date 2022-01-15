use proconio::marker::Usize1;
use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
        q: usize,
        tex: [(usize, Usize1, i64); q],
    }

    let mut g = vec![vec![]; n];
    for &(a, b) in ab.iter() {
        g[a].push(b);
        g[b].push(a);
    }

    let r = 0;

    let mut d = vec![0; n];
    let mut used = vec![false; n];
    let mut q = VecDeque::new();
    used[r] = true;
    q.push_back(r);
    while let Some(i) = q.pop_front() {
        for &j in g[i].iter() {
            if !used[j] {
                used[j] = true;
                d[j] = d[i] + 1;
                q.push_back(j);
            }
        }
    }

    let mut c = vec![0; n];

    for &(t, e, x) in tex.iter() {
        let (a, b) = ab[e];
        if t == 1 {
            if d[a] < d[b] {
                c[r] += x;
                c[b] -= x;
            } else {
                c[a] += x;
            }
        }
        if t == 2 {
            if d[b] < d[a] {
                c[r] += x;
                c[a] -= x;
            } else {
                c[b] += x;
            }
        }
    }

    let mut cs = vec![0; n];
    let mut q = VecDeque::new();
    cs[r] = c[r];
    q.push_back(r);
    while let Some(i) = q.pop_front() {
        for &j in g[i].iter() {
            if d[j] > d[i] {
                cs[j] = cs[i] + c[j];
                q.push_back(j);
            }
        }
    }

    for i in 0..n {
        println!("{}", cs[i]);
    }
}
