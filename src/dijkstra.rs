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
