use num_integer::Roots;
use proconio::{fastout, input, marker::Usize1};

const INF: u64 = std::u64::MAX;

#[allow(dead_code)]
fn dijkstra(graph: &Vec<Vec<(usize, u64, u64)>>, start: usize) -> Vec<u64> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    let mut dist = vec![INF; graph.len()];
    dist[start] = 0;
    heap.push(Reverse((0, start)));
    while let Some(Reverse((t, v))) = heap.pop() {
        if t > dist[v] {
            continue;
        }
        for &(w, c, d) in graph[v].iter() {
            let next = {
                let r = d.sqrt();
                ((r.max(100) - 100)..=(r + 100))
                    .filter(|&di| di >= t)
                    .map(|di| di + c + d / (di + 1))
                    .min()
                    .unwrap_or(INF)
                    .min(t + c + d / (t + 1))
            };
            if next < dist[w] {
                dist[w] = next;
                heap.push(Reverse((next, w)));
            }
        }
    }
    dist
}

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        abcd: [(Usize1, Usize1, u64, u64); m],
    }
    let mut g = vec![vec![]; n];
    for &(a, b, c, d) in abcd.iter() {
        if a == b {
            continue;
        }
        g[a].push((b, c, d));
        g[b].push((a, c, d));
    }
    let ans = dijkstra(&g, 0)[n - 1];
    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
