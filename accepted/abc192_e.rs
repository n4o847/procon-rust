use proconio::marker::Usize1;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize, x: Usize1, y: Usize1,
        abtk: [(Usize1, Usize1, u64, u64); m],
    }
    const INF: u64 = std::u64::MAX / 2;
    let g = {
        let mut g = vec![vec![]; n];
        for &(a, b, t, k) in abtk.iter() {
            g[a].push((b, t, k));
            g[b].push((a, t, k));
        }
        g
    };
    let ans = {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut que = BinaryHeap::new();
        let mut d = vec![INF; n];
        d[x] = 0;
        que.push(Reverse((0, x)));
        while let Some(Reverse(p)) = que.pop() {
            let v = p.1;
            let curr = p.0;
            if d[v] < curr {
                continue;
            }
            for &(w, t, k) in g[v].iter() {
                let next = (curr + k - 1) / k * k + t;
                if d[w] > next {
                    d[w] = next;
                    que.push(Reverse((next, w)));
                }
            }
        }
        d[y]
    };
    if ans >= INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
