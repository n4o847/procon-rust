use proconio::marker::Usize1;
use proconio::{fastout, input};
use std::collections::{BTreeSet, VecDeque};

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

    let mut s = 0;
    for i in 0..n {
        if g[i].len() == 1 {
            s = i;
            break;
        }
    }

    let mut v = vec![];
    let mut sz = vec![0; n];
    let mut used = vec![false; n];
    fn dfs(
        g: &Vec<Vec<usize>>,
        v: &mut Vec<usize>,
        sz: &mut Vec<usize>,
        used: &mut Vec<bool>,
        i: usize,
    ) -> usize {
        let mut s = 1;
        v.push(i);
        used[i] = true;
        for &j in g[i].iter() {
            if !used[j] {
                s += dfs(g, v, sz, used, j);
            }
        }
        sz[i] = s;
        s
    }
    dfs(&g, &mut v, &mut sz, &mut used, s);
    // dbg!(s, &v);
    // dbg!(s, &sz);

    let mut w = vec![0; n];
    for i in 0..n {
        w[v[i]] = i;
    }
    // dbg!(&w);

    // dbg!(&v);
    // dbg!(&sz);

    let mut cs = vec![0 as i64; n + 1];

    for &(t, e, x) in tex.iter() {
        let a = ab[e].0;
        let b = ab[e].1;
        assert_ne!(sz[a], sz[b]);
        // println!("{} {}", a, b);
        if t == 1 {
            if sz[a] > sz[b] {
                cs[0] += x;
                cs[w[b]] -= x;
                cs[w[b] + sz[b]] += x;
                cs[n] -= x;
            // println!(
            //     "add {} to {:?}",
            //     x,
            //     (0..w[a]).map(|i| v[i] + 1).collect::<Vec<_>>()
            // );
            } else {
                cs[w[a]] += x;
                cs[w[a] + sz[a]] -= x;
                // println!(
                //     "add {} to {:?}",
                //     x,
                //     (w[a]..w[a] + sz[a]).map(|i| v[i] + 1).collect::<Vec<_>>()
                // );
            }
        }
        if t == 2 {
            if sz[a] < sz[b] {
                cs[0] += x;
                cs[w[a]] -= x;
                cs[w[a] + sz[a]] += x;
                cs[n] -= x;
            // println!(
            //     "add {} to {:?}",
            //     x,
            //     (0..w[b]).map(|i| v[i] + 1).collect::<Vec<_>>()
            // );
            } else {
                cs[w[b]] += x;
                cs[w[b] + sz[b]] -= x;
                // println!(
                //     "add {} to {:?}",
                //     x,
                //     (w[b]..w[b] + sz[b]).map(|i| v[i] + 1).collect::<Vec<_>>()
                // );
            }
        }
    }

    let mut ac = vec![0];
    for i in 0..n {
        ac.push(ac[i] + cs[i]);
    }
    // dbg!(&cs);
    // dbg!(&ac);

    for i in 0..n {
        println!("{}", ac[w[i] + 1]);
    }
}
