use proconio::{fastout, input};

use proconio::marker::Usize1;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INF: u64 = std::u64::MAX >> 1;

fn dijkstra(g: &Vec<Vec<(usize, u64)>>, s: usize) -> Vec<u64> {
    let n = g.len();
    let mut d = vec![INF; n];
    let mut q = BinaryHeap::new();
    d[s] = 0;
    q.push(Reverse((0, s)));
    while let Some(Reverse((c, v))) = q.pop() {
        if d[v] < c {
            continue;
        }
        for &(dst, cost) in g[v].iter() {
            if d[dst] > d[v] + cost {
                d[dst] = d[v] + cost;
                q.push(Reverse((d[dst], dst)));
            }
        }
    }
    return d;
}

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        s: Usize1, t: Usize1,
        xyd: [(Usize1, Usize1, u64); m],
    }
    let mut g = vec![vec![]; n];
    for (x, y, d) in xyd {
        g[x].push((y, d));
        g[y].push((x, d));
    }
    let ds = dijkstra(&g, s);
    let dt = dijkstra(&g, t);
    for i in 0..n {
        if ds[i] == dt[i] && ds[i] != INF {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
