use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        abc: [(Usize1, Usize1, u64); m],
    }
    let mut g = vec![vec![]; n];
    for &(a, b, c) in abc.iter() {
        g[a].push((b, c));
        g[b].push((a, c));
    }
    let d1 = dijkstra(&g, 0);
    let dn = dijkstra(&g, n - 1);
    for k in 0..n {
        println!("{}", d1[k] + dn[k]);
    }
}

#[allow(dead_code)]
fn dijkstra(graph: &Vec<Vec<(usize, u64)>>, start: usize) -> Vec<u64> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    const INF: u64 = std::u64::MAX;
    let mut heap = BinaryHeap::new();
    let mut dist = vec![INF; graph.len()];
    dist[start] = 0;
    heap.push(Reverse((0, start)));
    while let Some(Reverse((c, v))) = heap.pop() {
        if c > dist[v] {
            continue;
        }
        for &(w, d) in graph[v].iter() {
            let next = dist[v] + d;
            if next < dist[w] {
                dist[w] = next;
                heap.push(Reverse((next, w)));
            }
        }
    }
    dist
}
