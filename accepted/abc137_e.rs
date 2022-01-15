use proconio::{fastout, input};

use proconio::marker::Usize1;

struct Edge {
    from: usize,
    to: usize,
    cost: i64,
}

#[fastout]
fn main() {
    const INF: i64 = 1 << 61;
    input! {
        n: usize, m: usize, p: i64,
        abc: [(Usize1, Usize1, i64); m],
    }
    let edges = abc
        .into_iter()
        .map(|(from, to, cost)| Edge {
            from,
            to,
            cost: p - cost,
        })
        .collect::<Vec<_>>();
    let mut d = vec![INF; n];
    d[0] = 0;
    for i in 0..(n * 2) {
        for e in edges.iter() {
            if d[e.from] < INF && d[e.to] > d[e.from] + e.cost {
                if i >= n - 1 {
                    if e.to == n - 1 {
                        println!("-1");
                        return;
                    } else {
                        d[e.to] = -INF;
                    }
                } else {
                    d[e.to] = d[e.from] + e.cost;
                }
            }
        }
    }
    let ans = std::cmp::max(0, -d[n - 1]);
    println!("{}", ans);
}
